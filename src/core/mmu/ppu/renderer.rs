/// Render is implemented to control drawing pixels to some output device.
/// This [Renderers] render function will be called everytime the PPU enters the
/// drawing mode. Note the gameboy typically runs at 60FPS it is up to implementation
/// of the render to control the frame rate. But note the render function should not
/// be blocked to do so (i.e. some form of threading is required).
pub trait Renderer: Send + Sync {
    /// Called with the updated buffer of pixels to be renderd to the device.
    /// Is called everytime the PPU enters drawing mode.
    fn render(&self, buffer: [u32; 160 * 144]);

    /// Function that converts a [Color] to a u32 value that can be used to draw
    /// to the output device. This function has a default implementation that
    /// returns a hexidecimal value for the color using a basic grey/white palette.
    fn palette(&self, color: Color) -> u32 {
        match color {
            Color::White => 0xFFFFFF,
            Color::LightGray => 0x454545,
            Color::DarkGray => 0xA8A8A8,
            Color::Black => 0,
        }
    }
}

#[cfg(test)]
pub struct TestRenderer;

#[cfg(test)]
impl Renderer for TestRenderer {
    fn render(&self, _: [u32; 160 * 144]) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}
