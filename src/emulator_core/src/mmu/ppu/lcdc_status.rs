/// The STAT register provides information about the status of the
/// LCD. Among its uses it allows the program to select the STAT interrupt
/// trigger and determine the PPU mode.
/// Bit 7 - Unused
/// Bit 6 - LYC=LY STAT Interrupt enable (when LY=LYC a STAT interrupt is triggered).
/// Bit 5 - Rendering Mode 2 STAT Interrupt enable (WHen entering Mode 2 --
///         STAT interrupt is triggered).
/// Bit 4 - Rendering Mode 1 STAT Interrupt enable (WHen entering Mode 1 --
///         STAT interrupt is triggered).
/// Bit 3 - Rendering Mode 0 STAT Interrupt enable (WHen entering Mode 0 --
///         STAT interrupt is triggered).
/// Bit 2 - Coincidence flag. Set if LY is currently equal to LYC.
/// Bit 1-0 - Current rendering mode (See [PPUMode] for details).
#[derive(Debug)]
pub struct LCDStatus(u8);

/// What mode is the PPU currently in.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PPUMode {
    // Mode 0
    HBlank,
    // Mode 1
    VBlank,
    // Mode 2
    Oam,
    // Mode 3
    Drawing,
}

impl From<u8> for PPUMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::HBlank,
            1 => Self::VBlank,
            2 => Self::Oam,
            3 => Self::Drawing,
            _ => unreachable!(),
        }
    }
}

impl From<PPUMode> for u8 {
    fn from(value: PPUMode) -> Self {
        match value {
            PPUMode::HBlank => 0,
            PPUMode::VBlank => 1,
            PPUMode::Oam => 2,
            PPUMode::Drawing => 3,
        }
    }
}

impl LCDStatus {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read(&self) -> u8 {
        self.0
    }

    pub fn write(&mut self, value: u8) {
        self.0 = value
    }

    /// Is LYC=LY STAT interrupt enabled
    pub fn lyc_ly_stat_ie(&self) -> bool {
        self.0 & 0b01000000 != 0
    }

    /// Should we trigger STAT interrupt when entering OAM
    pub fn oam_stat_ie(&self) -> bool {
        self.0 & 0b00100000 != 0
    }

    /// Should we trigger STAT interrupt when entering VBlank
    pub fn vblank_stat_ie(&self) -> bool {
        self.0 & 0b00010000 != 0
    }

    /// Should we trigger STAT interrupt when entering HBlank
    pub fn hblank_stat_ie(&self) -> bool {
        self.0 & 0b00001000 != 0
    }

    /// What is the current Mode of the PPU
    pub fn ppu_mode(&self) -> PPUMode {
        (self.0 & 0b00000011).into()
    }

    /// Update the PPU mode
    pub fn set_ppu_mode(&mut self, mode: PPUMode) {
        let value: u8 = mode.into();

        self.0 = (self.0 & !0x3) | value;
    }

    /// Updates LYC = LY
    pub fn set_lyc_eq_ly(&mut self, value: bool) {
        if value {
            self.0 |= 0b00000100
        } else {
            self.0 &= !0b00000100
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write() {
        let mut lcd_status = LCDStatus::new();

        lcd_status.write(100);

        assert_eq!(lcd_status.read(), 100);
    }

    #[test]
    fn lyc_ie_stat_ie() {
        let mut lcd_status = LCDStatus::new();
        lcd_status.write(0b01000000);
        assert!(lcd_status.lyc_ly_stat_ie());

        lcd_status.write(0b00000000);
        assert!(!lcd_status.lyc_ly_stat_ie());
    }

    #[test]
    fn oam_stat_ie() {
        let mut lcd_status = LCDStatus::new();
        lcd_status.write(0b00100000);
        assert!(lcd_status.oam_stat_ie());

        lcd_status.write(0b00000000);
        assert!(!lcd_status.oam_stat_ie());
    }

    #[test]
    fn vblank_stat_ie() {
        let mut lcd_status = LCDStatus::new();
        lcd_status.write(0b00010000);
        assert!(lcd_status.vblank_stat_ie());

        lcd_status.write(0b00000000);
        assert!(!lcd_status.vblank_stat_ie());
    }

    #[test]
    fn hblank_stat_ie() {
        let mut lcd_status = LCDStatus::new();
        lcd_status.write(0b00001000);
        assert!(lcd_status.hblank_stat_ie());

        lcd_status.write(0b00000000);
        assert!(!lcd_status.hblank_stat_ie());
    }
    #[test]
    fn ppu_mode() {
        let mut lcd_status = LCDStatus::new();

        lcd_status.set_ppu_mode(PPUMode::Oam);
        assert_eq!(lcd_status.ppu_mode(), PPUMode::Oam);

        lcd_status.set_ppu_mode(PPUMode::VBlank);
        assert_eq!(lcd_status.ppu_mode(), PPUMode::VBlank);

        lcd_status.set_ppu_mode(PPUMode::HBlank);
        assert_eq!(lcd_status.ppu_mode(), PPUMode::HBlank);

        lcd_status.set_ppu_mode(PPUMode::Drawing);
        assert_eq!(lcd_status.ppu_mode(), PPUMode::Drawing);
    }
}
