use super::tile::Pixel;

/// This struct is used for Object palette registers
/// which there are two of. It assigns gray shades to color ids
/// of pixels for sprites.

pub struct SpritePalette(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpritePaletteSelection {
    Palette0,
    Palette1,
}

impl SpritePalette {
    pub fn new() -> Self {
        Self(0)
    }

    pub(crate) fn read(&self) -> u8 {
        self.0
    }

    pub(crate) fn write(&mut self, value: u8) {
        self.0 = value
    }

    pub(super) fn color_from_pixel(&self, pixel: Pixel) -> SpriteColor {
        match pixel {
            Pixel::Color0 => SpriteColor::Transparent,
            Pixel::Color1 => self.color_1(),
            Pixel::Color2 => self.color_2(),
            Pixel::Color3 => self.color_3(),
        }
    }

    fn color_1(&self) -> SpriteColor {
        (self.0 >> 2 & 0b00000011).into()
    }

    fn color_2(&self) -> SpriteColor {
        (self.0 >> 4 & 0b00000011).into()
    }

    fn color_3(&self) -> SpriteColor {
        (self.0 >> 6 & 0b00000011).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpriteColor {
    White,
    LightGray,
    DarkGray,
    Black,
    Transparent,
}

impl From<u8> for SpriteColor {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_write() {
        let mut palette = SpritePalette::new();

        palette.write(20);

        assert_eq!(palette.read(), 20);
    }

    #[test]
    fn color_from_pixel() {
        let mut palette = SpritePalette::new();
        palette.write(0b10110010);

        assert_eq!(
            palette.color_from_pixel(Pixel::Color0),
            SpriteColor::Transparent
        );

        assert_eq!(palette.color_from_pixel(Pixel::Color1), SpriteColor::White);

        assert_eq!(palette.color_from_pixel(Pixel::Color2), SpriteColor::Black);

        assert_eq!(
            palette.color_from_pixel(Pixel::Color3),
            SpriteColor::DarkGray
        );
    }
}
