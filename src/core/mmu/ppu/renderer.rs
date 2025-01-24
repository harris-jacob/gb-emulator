use super::Pixel;

/// Render is implemented to control drawing pixels to some output device.
/// This [Renderers] render function will be called everytime the PPU enters the
/// drawing mode. Note the gameboy typically runs at 60FPS it is up to implementation
/// of the render to control the frame rate. But note the render function should not
/// be blocked to do so (i.e. some form of threading is required).
pub trait Renderer {
    /// Called with the updated buffer of pixels to be renderd to the device.
    /// Is called everytime the PPU enters drawing mode.
    fn render(&mut self, buffer: [Pixel; 160 * 144]);
}

#[cfg(test)]
pub struct TestRenderer;

#[cfg(test)]
impl Renderer for TestRenderer {
    fn render(&mut self, _: [Pixel; 160 * 144]) {}
}
