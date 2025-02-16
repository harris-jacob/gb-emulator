/// The background layer is 256x256 but the LCD is only 160x144.
/// So the [BackgroundViewport] controls which 'slice' of the background
/// is displayed. It houses two registers:
/// SCX - Controls X position of the origin of the viewport (top-left).
/// SCY - Controls Y position of the origin of the viewport (top-left).
pub struct BackgroundViewport {
    pub scx: u8,
    pub scy: u8,
}

impl Default for BackgroundViewport {
    fn default() -> Self {
        Self { scx: 0, scy: 0 }
    }
}

pub enum ViewportRegister {
    SCX,
    SCY,
}
