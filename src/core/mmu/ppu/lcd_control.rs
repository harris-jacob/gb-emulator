pub struct LCDControl(u8);

/// Each bit in this register controls state of the LCD and its rendering
/// behaviour.
/// Bit 7 - LCD Display Enable (0=Off, 1=On)
/// Bit 6 - Chooses which background map the window uses (0=9800-9BFF, 1=9C00-9FFF)
/// Bit 5 - Window Display Enable (0=Off, 1=On). Controls whether the window will be displayed
/// Bit 4 - Chooses which addressing mode the tile data uses (0=Signed, 1=Unsigned)
/// Bit 3 - Chooses which background map the background uses (0=9800-9BFF, 1=9C00-9FFF)
/// Bit 2 - Sprite Size (0=8x8, 1=8x16)
/// Bit 1 - Sprite Display Enable (0=Off, 1=On). Controls whether sprites will be displayed
/// Bit 0 - Background/Window Display (0=Off, 1=On). If off background and window are 
/// not rendered.
impl LCDControl {
    pub new() -> Self {
        Self(0)
    }
}
