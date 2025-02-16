use super::Color;
use super::Pixel;

/// This register assigns gray shades to the color ids of the BG and window
/// tiles.
pub struct BackgroundPalette(u8);

impl BackgroundPalette {
    pub fn new() -> Self {
        Self(0)
    }

    pub(crate) fn read(&self) -> u8 {
        self.0
    }

    pub(crate) fn write(&mut self, value: u8) {
        self.0 = value
    }

    pub(super) fn color_from_pixel(&self, pixel: Pixel) -> BackgroundColor {
        match pixel {
            Pixel::Color0 => self.color_0(),
            Pixel::Color1 => self.color_1(),
            Pixel::Color2 => self.color_2(),
            Pixel::Color3 => self.color_3(),
        }
    }

    fn color_0(&self) -> BackgroundColor {
        (self.0 & 0b0000011).into()
    }

    fn color_1(&self) -> BackgroundColor {
        (self.0 >> 2 & 0b00000011).into()
    }

    fn color_2(&self) -> BackgroundColor {
        (self.0 >> 4 & 0b00000011).into()
    }

    fn color_3(&self) -> BackgroundColor {
        (self.0 >> 6 & 0b00000011).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BackgroundColor {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl From<u8> for BackgroundColor {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::White,
            1 => Self::LightGray,
            2 => Self::DarkGray,
            3 => Self::Black,
            _ => unreachable!(),
        }
    }
}

// TODO: remove this when proper pixel mixing is implemented
impl From<BackgroundColor> for Color {
    fn from(value: BackgroundColor) -> Self {
        match value {
            BackgroundColor::White => Color::White,
            BackgroundColor::LightGray => Color::LightGray,
            BackgroundColor::DarkGray => Color::DarkGray,
            BackgroundColor::Black => Color::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_write() {
        let mut palette = BackgroundPalette::new();

        palette.write(20);

        assert_eq!(palette.read(), 20);
    }

    #[test]
    fn color_from_pixel() {
        let mut palette = BackgroundPalette::new();
        palette.write(0b01001110);

        assert_eq!(
            palette.color_from_pixel(Pixel::Color0),
            BackgroundColor::DarkGray
        );

        assert_eq!(
            palette.color_from_pixel(Pixel::Color1),
            BackgroundColor::Black
        );

        assert_eq!(
            palette.color_from_pixel(Pixel::Color2),
            BackgroundColor::White
        );

        assert_eq!(
            palette.color_from_pixel(Pixel::Color3),
            BackgroundColor::LightGray
        );
    }
}
