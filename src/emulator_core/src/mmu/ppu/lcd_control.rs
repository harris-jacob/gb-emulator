use super::*;

/// Each bit in this register controls state of the LCD and its rendering
/// behaviour.
/// Bit 7 - LCD Display Enable (0=Off, 1=On)
/// Bit 6 - Chooses which background map the window uses (0=9800-9BFF, 1=9C00-9FFF)
/// Bit 5 - Window Display Enable (0=Off, 1=On). Controls whether the window will be displayed
/// Bit 4 - Chooses which addressing mode the backgroun/window uses for tile data (0=Signed, 1=Unsigned)
/// Bit 3 - Chooses which background map the background uses (0=9800-9BFF, 1=9C00-9FFF)
/// Bit 2 - Sprite Size (0=8x8, 1=8x16)
/// Bit 1 - Sprite Display Enable (0=Off, 1=On). Controls whether sprites will be displayed
/// Bit 0 - Background/Window Display (0=Off, 1=On). If off background and window are
/// not rendered.
pub struct LCDControl(u8);

impl LCDControl {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read(&self) -> u8 {
        self.0
    }

    pub(crate) fn write(&mut self, value: u8) {
        self.0 = value
    }

    /// Is the LCD display enabled. If off the PPU doesn't run
    pub fn lcd_enable(&self) -> bool {
        self.0 & 0b10000000 != 0
    }

    /// Which Background map the window layer uses.
    pub fn window_background_map(&self) -> BGMapSelection {
        if self.0 & 0b01000000 == 0 {
            BGMapSelection::Map0
        } else {
            BGMapSelection::Map1
        }
    }

    /// Is the window layer enabled.
    pub fn window_enabled(&self) -> bool {
        self.0 & 0b00100000 != 0
    }

    /// Which addressing method the window and background layers should use.
    pub fn addressing_method(&self) -> TileAddressingMethod {
        if self.0 & 0b00010000 == 0 {
            TileAddressingMethod::Signed
        } else {
            TileAddressingMethod::Unsigned
        }
    }

    /// Which Background map the background layer uses.
    pub fn background_background_map(&self) -> BGMapSelection {
        if self.0 & 0b00001000 == 0 {
            BGMapSelection::Map0
        } else {
            BGMapSelection::Map1
        }
    }

    /// Size of the sprite to draw.
    pub fn sprite_size(&self) -> SpriteSize {
        if self.0 & 0b00000100 == 0 {
            SpriteSize::Normal
        } else {
            SpriteSize::Long
        }
    }

    /// Is the sprite display enabled? If not, sprites are not rendered
    pub fn sprites_enabled(&self) -> bool {
        self.0 & 0b00000010 != 0
    }

    /// If false, the background and window layers are disabled
    pub fn background_and_window_enabled(&self) -> bool {
        self.0 & 0b00000001 != 0
    }

    /// Returns true when a new write to the LCDC buffer (value)
    /// would cause the LCD enable bit to be toggled off.
    /// When this happens some state needs to reset in the PPU.
    pub fn is_lcd_enable_toggled_off(&self, value: u8) -> bool {
        let is_new_lcd_enable = value & 0b10000000 == 128;

        !is_new_lcd_enable && self.lcd_enable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write() {
        let mut lcdc = LCDControl::new();

        lcdc.write(100);

        assert_eq!(lcdc.read(), 100);
    }

    #[test]
    fn lcd_enable() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b10000000);
        assert!(lcdc.lcd_enable());

        lcdc.write(0b00000000);
        assert!(!lcdc.lcd_enable());
    }

    #[test]
    fn window_background_map() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b01000000);
        assert_eq!(lcdc.window_background_map(), BGMapSelection::Map1);

        lcdc.write(0b00000000);
        assert_eq!(lcdc.window_background_map(), BGMapSelection::Map0);
    }

    #[test]
    fn window_enabled() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b00100000);
        assert!(lcdc.window_enabled());

        lcdc.write(0b00000000);
        assert!(!lcdc.window_enabled());
    }

    #[test]
    fn addressing_method() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b00010000);
        assert_eq!(lcdc.addressing_method(), TileAddressingMethod::Unsigned);

        lcdc.write(0b00000000);
        assert_eq!(lcdc.addressing_method(), TileAddressingMethod::Signed);
    }

    #[test]
    fn background_background_map() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b00001000);
        assert_eq!(lcdc.background_background_map(), BGMapSelection::Map1);

        lcdc.write(0b00000000);
        assert_eq!(lcdc.background_background_map(), BGMapSelection::Map0);
    }

    #[test]
    fn sprite_size() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b00000100);
        assert_eq!(lcdc.sprite_size(), SpriteSize::Long);

        lcdc.write(0b00000000);
        assert_eq!(lcdc.sprite_size(), SpriteSize::Normal);
    }

    #[test]
    fn sprites_enabled() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b00000010);
        assert!(lcdc.sprites_enabled());

        lcdc.write(0b00000000);
        assert!(!lcdc.sprites_enabled());
    }

    #[test]
    fn background_and_window_enabled() {
        let mut lcdc = LCDControl::new();
        lcdc.write(0b00000001);
        assert!(lcdc.background_and_window_enabled());

        lcdc.write(0b00000000);
        assert!(!lcdc.background_and_window_enabled());
    }
}
