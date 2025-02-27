mod background_map;
mod background_palette;
mod background_tile;
mod background_viewport;
mod lcd_control;
pub mod lcdc_status;
mod oam;
mod pixel;
mod renderer;
mod rendering;
mod sprite_palette;
mod sprite_tile;
mod tiledata;
mod window_position;

use std::sync::Arc;

use background_map::BackgroundMap;
use background_palette::BackgroundPalette;
use background_tile::BackgroundTile;
use background_viewport::BackgroundViewport;
use lcd_control::LCDControl;
use lcdc_status::LCDStatus;
use oam::SpriteFlags;
use oam::SpriteSize;
use oam::OAM;
use pixel::Pixel;
use sprite_palette::SpritePalette;
use sprite_tile::SpriteTile;
use tiledata::TileData;
use window_position::WindowPosition;

pub use background_map::BGMapSelection;
pub use background_viewport::ViewportRegister;
pub use renderer::Color;
pub use renderer::Renderer;
pub use sprite_palette::SpritePaletteSelection;
pub use tiledata::TileAddressingMethod;
pub use window_position::WindowPositionRegister;

#[cfg(test)]
pub use renderer::TestRenderer;

pub const HEIGHT: usize = 144;
pub const WIDTH: usize = 160;

pub struct PPU {
    pub interrupt_request: InterruptRequests,
    background_viewport: BackgroundViewport,
    background_palette: BackgroundPalette,
    pub buffer: [u32; WIDTH * HEIGHT],
    // TODO: refactor to eliminate this
    bg_priority: [Pixel; WIDTH * HEIGHT],
    bg_map0: BackgroundMap,
    bg_map1: BackgroundMap,
    clock: u32,
    lcd_stat: LCDStatus,
    lcdc: LCDControl,
    ly: u8,
    lyc: u8,
    oam: OAM,
    sprite_palette_0: SpritePalette,
    sprite_palette_1: SpritePalette,
    tiledata: TileData,
    window_position: WindowPosition,
    renderer: Arc<dyn Renderer>,
}

#[derive(Default)]
pub struct InterruptRequests {
    pub vblank: bool,
    pub stat: bool,
}

impl PPU {
    pub fn new(renderer: Arc<dyn Renderer>) -> Self {
        Self {
            background_viewport: BackgroundViewport::default(),
            background_palette: BackgroundPalette::new(),
            bg_priority: [Pixel::Color0; WIDTH * HEIGHT],
            bg_map0: BackgroundMap::new(),
            bg_map1: BackgroundMap::new(),
            buffer: [renderer.palette(Color::Black); WIDTH * HEIGHT],
            clock: 0,
            interrupt_request: InterruptRequests::default(),
            lcd_stat: LCDStatus::new(),
            lcdc: LCDControl::new(),
            ly: 0,
            lyc: 0,
            oam: OAM::new(),
            sprite_palette_0: SpritePalette::new(),
            sprite_palette_1: SpritePalette::new(),
            tiledata: TileData::new(),
            window_position: WindowPosition::default(),
            renderer,
        }
    }

    /// Read from the SCX, SCY registers.
    pub(crate) fn read_background_viewport(&self, viewport_register: ViewportRegister) -> u8 {
        match viewport_register {
            ViewportRegister::SCX => self.background_viewport.scx,
            ViewportRegister::SCY => self.background_viewport.scy,
        }
    }

    /// Write from the SCX, SCY registers.
    pub(crate) fn write_background_viewport(
        &mut self,
        viewport_register: ViewportRegister,
        value: u8,
    ) {
        match viewport_register {
            ViewportRegister::SCX => self.background_viewport.scx = value,
            ViewportRegister::SCY => self.background_viewport.scy = value,
        }
    }

    /// Read from the background palette register.
    pub(crate) fn read_background_palette(&self) -> u8 {
        self.background_palette.read()
    }

    /// Write to the background palette register.
    pub(crate) fn write_background_palette(&mut self, value: u8) {
        self.background_palette.write(value)
    }

    /// Read from one of the background maps. Each background map accepts
    /// an address in the range: 0-0x3FE (inclusive).
    pub(crate) fn read_bg_map(&self, bgmap: BGMapSelection, addr: u16) -> u8 {
        match bgmap {
            BGMapSelection::Map0 => self.bg_map0.read(addr),
            BGMapSelection::Map1 => self.bg_map1.read(addr),
        }
    }

    /// Write from one of the background maps. Each background map accepts
    /// an address in the range: 0-0x3FE (inclusive).
    pub(crate) fn write_bg_map(&mut self, bgmap: BGMapSelection, addr: u16, value: u8) {
        match bgmap {
            BGMapSelection::Map0 => self.bg_map0.write(addr, value),
            BGMapSelection::Map1 => self.bg_map1.write(addr, value),
        }
    }

    /// Read from the LCD stat register
    pub(crate) fn read_lcd_stat(&self) -> u8 {
        self.lcd_stat.read()
    }

    /// Write to the LCD stat register.
    pub(crate) fn write_lcd_stat(&mut self, value: u8) {
        self.lcd_stat.write(value)
    }

    /// Read from the LCD control register
    pub(crate) fn read_lcdc(&self) -> u8 {
        self.lcdc.read()
    }

    /// Read from the LCD control register
    pub(crate) fn write_lcdc(&mut self, value: u8) {
        if self.lcdc.is_lcd_enable_toggled_off(value) {
            self.update_ly(0);
            self.lcd_stat.set_ppu_mode(lcdc_status::PPUMode::HBlank);
            self.clock = 0;
            self.reset_buffer();
        }

        self.lcdc.write(value)
    }

    /// Read from one of the 2 available sprite palettes
    pub(crate) fn read_sprite_palette(&self, palette: SpritePaletteSelection) -> u8 {
        match palette {
            SpritePaletteSelection::Palette0 => self.sprite_palette_0.read(),
            SpritePaletteSelection::Palette1 => self.sprite_palette_1.read(),
        }
    }

    /// Write to one of the 2 available sprite palettes
    pub(crate) fn write_sprite_palette(&mut self, palette: SpritePaletteSelection, value: u8) {
        match palette {
            SpritePaletteSelection::Palette0 => self.sprite_palette_0.write(value),
            SpritePaletteSelection::Palette1 => self.sprite_palette_1.write(value),
        }
    }

    /// Read from tiledata. Accepts addresses in the range: 0-17FF (inclusive)
    pub(crate) fn read_tiledata(&self, addr: u16) -> u8 {
        self.tiledata.read(addr)
    }

    /// Write to tiledata. Accepts addresses in the range: 0-17FF (inclusive)
    pub(crate) fn write_tiledata(&mut self, addr: u16, value: u8) {
        self.tiledata.write(addr, value)
    }

    /// Read from OAM (sprite data). Accepts addresses in the range: 0-159 (inclusive)
    pub(crate) fn read_oam(&self, addr: u16) -> u8 {
        self.oam.read(addr)
    }

    /// write to OAM (sprite data). Accepts addresses in the range: 0-159 (inclusive)
    pub(crate) fn write_oam(&mut self, addr: u16, value: u8) {
        self.oam.write(addr, value)
    }

    /// Read from one of the window position registers (WX, WY)
    pub(crate) fn read_window_position(&self, register: WindowPositionRegister) -> u8 {
        match register {
            WindowPositionRegister::WX => self.window_position.wx,
            WindowPositionRegister::WY => self.window_position.wy,
        }
    }

    /// Read from the LYC register
    pub(crate) fn read_lyc(&self) -> u8 {
        self.lyc
    }

    /// Write to the LYC register
    pub(crate) fn write_lyc(&mut self, value: u8) {
        self.lyc = value;
        self.ly_compare();
    }

    /// Read from the LY register
    pub(crate) fn read_ly(&self) -> u8 {
        self.ly
    }

    /// Write to one of the window position registers (WX, WY)
    pub(crate) fn write_window_position(&mut self, register: WindowPositionRegister, value: u8) {
        match register {
            WindowPositionRegister::WX => self.window_position.wx = value,
            WindowPositionRegister::WY => self.window_position.wy = value,
        }
    }

    /// Compare Ly and Lyc register, setting coincidence flag and requesting
    /// STAT interrupt if enabled
    pub(super) fn ly_compare(&mut self) {
        self.lcd_stat.set_lyc_eq_ly(self.ly == self.lyc);

        if self.lcd_stat.lyc_ly_stat_ie() && self.lyc == self.ly {
            self.request_stat_interrupt()
        }
    }

    pub(super) fn request_stat_interrupt(&mut self) {
        self.interrupt_request.stat = true
    }

    pub(super) fn request_vblank_interrupt(&mut self) {
        self.interrupt_request.vblank = true
    }

    fn reset_buffer(&mut self) {
        self.buffer = [self.renderer.palette(Color::Black); 160 * 144];
    }
}
