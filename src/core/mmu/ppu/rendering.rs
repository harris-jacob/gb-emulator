use lcdc_status::PPUMode;

use super::*;

// TODO: double check these
const HBLANK_CYCLES: u32 = 204 / 4;
const VBLANK_CYCLES: u32 = 456 / 4;
const DRAWING_CYCLES: u32 = 172 / 4;
const OAM_CYCLES: u32 = 80 / 4;

impl PPU {
    /// Run a single rendering step of the PPU for the given
    /// amount of clock cycles.
    pub(crate) fn step(&mut self, cycles: u8) {
        if !self.lcdc.lcd_enable() {
            return;
        }

        self.update_clock(cycles);

        match self.lcd_stat.ppu_mode() {
            PPUMode::HBlank => self.hblank_step(),
            PPUMode::VBlank => self.vblank_step(),
            PPUMode::OAM => self.oam_step(),
            PPUMode::Drawing => self.drawing_step(),
        }
    }

    pub(super) fn update_ly(&mut self, value: u8) {
        self.ly = value;
        self.lcd_stat.set_lyc_eq_ly(self.ly == self.lyc);

        if self.lcd_stat.lyc_ly_stat_ie() {
            self.request_stat_interrupt()
        }
    }

    fn hblank_step(&mut self) {
        if self.clock < HBLANK_CYCLES {
            return;
        }

        self.clock %= HBLANK_CYCLES;

        if self.ly < 143 {
            self.update_ly(self.ly + 1);
            self.switch_mode(PPUMode::OAM)
        } else {
            self.renderer.render(self.buffer);
            self.reset_buffer();
            self.switch_mode(PPUMode::VBlank)
        }
    }

    fn vblank_step(&mut self) {
        if self.clock < VBLANK_CYCLES {
            return;
        }

        self.clock %= VBLANK_CYCLES;
        self.update_ly(self.ly + 1);

        if self.ly >= 154 {
            self.update_ly(0);
            self.switch_mode(PPUMode::OAM)
        }
    }

    fn oam_step(&mut self) {
        if self.clock < OAM_CYCLES {
            return;
        }

        self.clock %= OAM_CYCLES;
        self.switch_mode(PPUMode::Drawing);
    }

    fn drawing_step(&mut self) {
        if self.clock < DRAWING_CYCLES {
            return;
        }

        self.clock %= DRAWING_CYCLES;
        self.render_background_scanline();
        self.switch_mode(PPUMode::HBlank);
    }

    fn switch_mode(&mut self, mode: PPUMode) {
        match mode {
            PPUMode::HBlank => {
                if self.lcd_stat.hblank_stat_ie() {
                    self.request_stat_interrupt()
                }

                self.lcd_stat.set_ppu_mode(PPUMode::HBlank)
            }
            PPUMode::VBlank => {
                if self.lcd_stat.vblank_stat_ie() {
                    self.request_stat_interrupt()
                }

                self.request_vblank_interrupt();

                self.lcd_stat.set_ppu_mode(PPUMode::VBlank)
            }
            PPUMode::OAM => {
                if self.lcd_stat.oam_stat_ie() {
                    self.request_stat_interrupt()
                }

                self.lcd_stat.set_ppu_mode(PPUMode::OAM)
            }
            PPUMode::Drawing => self.lcd_stat.set_ppu_mode(PPUMode::Drawing),
        }
    }

    fn render_background_scanline(&mut self) {
        if !self.lcdc.background_and_window_enabled() {
            return;
        }

        for x in 0..WIDTH {
            self.render_background_layer_pixel(x as u8, self.ly);
        }
    }

    fn render_background_layer_pixel(&mut self, x: u8, y: u8) {
        let bg_map = self.current_background_map();

        let tile_x = x.wrapping_add(self.background_viewport.scx);
        let tile_y = y.wrapping_add(self.background_viewport.scy);

        let tile_number = bg_map.tile_number_at(tile_x, tile_y);
        // if tile_number != 0 && tile_number != 0x32 {
        //     println!("tile_number: {}", tile_number);
        //     println!("tile_x: {}", tile_x);
        //     println!("tile_y: {}", tile_y);
        //     panic!()
        // }
        let addressing_method = self.lcdc.addressing_method();

        let tile = self.tiledata.tile_at(tile_number, addressing_method);

        let pixel_x = tile_x % 8;
        let pixel_y = tile_y % 8;

        let pixel = tile.pixel_at(pixel_x, pixel_y);

        let color = self.background_palette.color_from_pixel(pixel);

        self.buffer[y as usize * WIDTH + x as usize] = self.renderer.palette(color.into());
    }

    fn current_background_map(&self) -> &BackgroundMap {
        match self.lcdc.background_background_map() {
            BGMapSelection::Map0 => &self.bg_map0,
            BGMapSelection::Map1 => &self.bg_map1,
        }
    }

    fn current_window_map(&self) -> &BackgroundMap {
        match self.lcdc.window_background_map() {
            BGMapSelection::Map0 => &self.bg_map0,
            BGMapSelection::Map1 => &self.bg_map1,
        }
    }

    fn update_clock(&mut self, cycles: u8) {
        self.clock += cycles as u32;
    }

    fn request_stat_interrupt(&mut self) {
        println!("Requesting STAT interrupt");
        self.interrupt_request.stat = true
    }

    fn request_vblank_interrupt(&mut self) {
        self.interrupt_request.vblank = true
    }
}
