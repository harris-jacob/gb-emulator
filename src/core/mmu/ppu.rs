mod background_map;
mod background_viewport;
mod lcd_control;
mod lcdc_status;
mod oam;
mod rendering;
mod tile;
mod tiledata;
mod window_position;

use background_map::BackgroundMap;
use background_viewport::BackgroundViewport;
use lcd_control::LCDControl;
use lcdc_status::LCDStatus;
use oam::OAM;
use tile::Pixel;
use tile::Tile;
use tiledata::TileData;
use window_position::WindowPosition;

pub use background_map::BackgroundMapSelection;
pub use background_viewport::ViewportRegister;
pub use oam::SpriteSize;
pub use tiledata::TileAddressingMethod;
pub use window_position::WindowPositionRegister;

pub struct PPU {
    pub interrupt_request: InterruptRequests,
    background_viewport: BackgroundViewport,
    bg_map0: BackgroundMap,
    bg_map1: BackgroundMap,
    clock: u32,
    lcd_stat: LCDStatus,
    lcdc: LCDControl,
    ly: u8,
    lyc: u8,
    oam: OAM,
    tiledata: TileData,
    window_position: WindowPosition,
}

/// TODO: this can be a more compact type
/// with an API that matches the timer interrupt
pub struct InterruptRequests {
    pub vblank: bool,
    pub stat: bool,
}

impl Default for InterruptRequests {
    fn default() -> Self {
        Self {
            vblank: false,
            stat: false,
        }
    }
}

impl PPU {
    pub fn new() -> Self {
        Self {
            background_viewport: BackgroundViewport::default(),
            bg_map0: BackgroundMap::new(),
            bg_map1: BackgroundMap::new(),
            clock: 0,
            interrupt_request: InterruptRequests::default(),
            lcd_stat: LCDStatus::new(),
            lcdc: LCDControl::new(),
            ly: 0,
            lyc: 0,
            oam: OAM::new(),
            tiledata: TileData::new(),
            window_position: WindowPosition::default(),
        }
    }

    /// Read from the SCX, SCY registers.
    pub fn read_background_viewport(&self, viewport_register: ViewportRegister) -> u8 {
        match viewport_register {
            ViewportRegister::SCX => self.background_viewport.scx,
            ViewportRegister::SCY => self.background_viewport.scy,
        }
    }

    /// Write from the SCX, SCY registers.
    pub fn write_background_viewport(&mut self, viewport_register: ViewportRegister, value: u8) {
        match viewport_register {
            ViewportRegister::SCX => self.background_viewport.scx = value,
            ViewportRegister::SCY => self.background_viewport.scy = value,
        }
    }

    /// Read from one of the background maps. Each background map accepts
    /// an address in the range: 0-0x3FE (inclusive).
    pub fn read_bg_map(&self, bgmap: BackgroundMapSelection, addr: u16) -> u8 {
        match bgmap {
            BackgroundMapSelection::Map0 => self.bg_map0.read(addr),
            BackgroundMapSelection::Map1 => self.bg_map1.read(addr),
        }
    }

    /// Write from one of the background maps. Each background map accepts
    /// an address in the range: 0-0x3FE (inclusive).
    pub fn write_bg_map(&mut self, bgmap: BackgroundMapSelection, addr: u16, value: u8) {
        match bgmap {
            BackgroundMapSelection::Map0 => self.bg_map0.write(addr, value),
            BackgroundMapSelection::Map1 => self.bg_map1.write(addr, value),
        }
    }

    /// Read from the LCD stat register
    pub fn read_lcd_stat(&self) -> u8 {
        self.lcd_stat.read()
    }

    /// Write to the LCD stat register.
    pub fn write_lcd_stat(&mut self, value: u8) {
        self.lcd_stat.write(value)
    }

    /// Read from the LCD control register
    pub fn read_lcdc(&self) -> u8 {
        self.lcdc.read()
    }

    /// Read from the LCD control register
    pub fn write_lcdc(&mut self, value: u8) {
        // TODO: this should reset a bunch of stuff if the LCD is toggled
        self.lcdc.write(value)
    }

    /// Read from OAM (sprite data). Accepts addresses in the range: 0-159 (inclusive)
    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam.read(addr)
    }

    /// write to OAM (sprite data). Accepts addresses in the range: 0-159 (inclusive)
    pub fn write_oam(&mut self, addr: u16, value: u8) {
        self.oam.write(addr, value)
    }

    /// Read from one of the window position registers (WX, WY)
    pub fn read_window_position(&self, register: WindowPositionRegister) -> u8 {
        match register {
            WindowPositionRegister::WX => self.window_position.wx,
            WindowPositionRegister::WY => self.window_position.wy,
        }
    }

    /// Read from the LYC register
    pub fn read_lyc(&self) -> u8 {
        self.lyc
    }

    /// Write to the LYC register
    pub fn write_lyc(&mut self, value: u8) {
        self.lyc = value
    }

    /// Read from the LY register
    pub fn read_ly(&self) -> u8 {
        self.ly
    }

    /// Write to one of the window position registers (WX, WY)
    pub fn write_window_position(&mut self, register: WindowPositionRegister, value: u8) {
        match register {
            WindowPositionRegister::WX => self.window_position.wx = value,
            WindowPositionRegister::WY => self.window_position.wy = value,
        }
    }
}
