#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// A pixel as defined by a Tile's 2bpp representation
/// i.e. 00 -> Color0, 01 -> Color1, 10 -> Color2, 11 -> Color 3
/// Should only ever be constructed from 2bits, otherwise it will
/// panic.
pub enum Pixel {
    Color0,
    Color1,
    Color2,
    Color3,
}

impl From<u8> for Pixel {
    fn from(value: u8) -> Self {
        match value {
            0 => Pixel::Color0,
            1 => Pixel::Color1,
            2 => Pixel::Color2,
            3 => Pixel::Color3,
            _ => unreachable!(),
        }
    }
}
