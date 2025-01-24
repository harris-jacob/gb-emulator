use lcdc_status::PPUMode;

use super::*;

// TODO: double check these
const HBLANK_CYCLES: u32 = 204 / 4;
const VBLANK_CYCLES: u32 = 456 / 4;
const DRAWING_CYCLS: u32 = 172 / 4;
const OAM_CYCLES: u32 = 80 / 4;

impl PPU {
    /// Run a single rendering step of the PPU for the given
    /// amount of clock cycles.
    pub(crate) fn step(&mut self, cycles: u8) {
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

        if self.ly <= 143 {
            self.update_ly(self.ly + 1);
            self.switch_mode(PPUMode::OAM)
        } else {
            self.switch_mode(PPUMode::VBlank)
        }
    }

    fn vblank_step(&mut self) {
        if self.clock < VBLANK_CYCLES {
            return;
        }

        self.clock %= VBLANK_CYCLES;

        if self.ly < 153 {
            self.update_ly(self.ly + 1);
        } else {
            self.update_ly(0);
            self.switch_mode(PPUMode::OAM);
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
        if self.clock < DRAWING_CYCLS {
            return;
        }

        self.clock %= DRAWING_CYCLS;
        self.renderer.render(self.buffer);
        self.reset_buffer();
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

    fn update_clock(&mut self, cycles: u8) {
        self.clock += cycles as u32;
    }

    fn request_stat_interrupt(&mut self) {
        self.interrupt_request.stat = true
    }

    fn request_vblank_interrupt(&mut self) {
        self.interrupt_request.vblank = true
    }
}
