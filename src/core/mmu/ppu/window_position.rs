/// Houses the window position registers, which control the X,Y position of the
/// window origin (top left). This struct holds two registers:
/// WX - is available at $FF4B and controls the position of the window horizontally.
/// WY is available at $FF4A and controls the position of the window vertically.
pub struct WindowPosition {
    pub wx: u8,
    pub wy: u8,
}

pub enum WindowPositionRegister {
    WX,
    WY,
}

impl Default for WindowPosition {
    fn default() -> Self {
        Self { wx: 0, wy: 0 }
    }
}

impl WindowPosition {
    // Accessor for the PPU to get the 'processed' value
    // of the register for rendering
    pub(super) fn wy(&self) -> u8 {
        self.wy
    }

    // Accessor for the PPU to get the 'processed' value
    // of the register for rendering. For WX this means
    // subtracting 7 from stored value (clamped to 0).
    // Since the window can only render pixels where X >= 7.
    pub(super) fn wx(&self) -> u8 {
        self.wx.checked_sub(7).unwrap_or(0)
    }
}
