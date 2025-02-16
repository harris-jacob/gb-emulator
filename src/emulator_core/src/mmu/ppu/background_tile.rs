use super::Pixel;

/// A background tile is a 8x8 pixel image encoded using the 2bpp representation.
/// This type handles the nitty-gritty of the encoding and allows access to
/// the pixel color information at given x,y co-ordinates.
#[derive(Debug)]
pub struct BackgroundTile<'a>(&'a [u8]);

impl<'a> BackgroundTile<'a> {
    pub fn new(data: &'a [u8]) -> BackgroundTile<'a> {
        Self(data)
    }

    /// Get the color of the pixel at the given x,y coordinate.
    /// Since tiles are 8x8 this function panics if y|x are
    /// larger than 7.
    pub fn pixel_at(&self, x: u8, y: u8) -> Pixel {
        Self::check_coord_range(x, y);

        let line_first_byte = self.idx(y * 2);
        let line_second_byte = self.idx(y * 2 + 1);

        let bit_one = line_first_byte >> (7 - x) & 0b1;
        let bit_two = line_second_byte >> (7 - x) & 0b1;

        let pixel = (bit_two << 1) | bit_one;

        pixel.into()
    }

    fn idx(&self, idx: u8) -> u8 {
        self.0[idx as usize]
    }

    fn check_coord_range(x: u8, y: u8) {
        if y >= 8 || x >= 8 {
            panic!("pixel out of range")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn tile_at_panics_reading_out_of_range_coords() {
        let tile = BackgroundTile::new(&[0; 16]);

        tile.pixel_at(9, 9);
    }

    #[test]
    fn pixel_at_returns_correct_pixel() {
        let face = smiley_face();
        let tile = BackgroundTile::new(&face);

        assert_eq!(tile.pixel_at(2, 1), Pixel::Color1);
        assert_eq!(tile.pixel_at(2, 2), Pixel::Color1);
        assert_eq!(tile.pixel_at(5, 1), Pixel::Color2);
        assert_eq!(tile.pixel_at(5, 2), Pixel::Color2);
        assert_eq!(tile.pixel_at(2, 5), Pixel::Color3);
        assert_eq!(tile.pixel_at(5, 5), Pixel::Color3);
        assert_eq!(tile.pixel_at(3, 6), Pixel::Color3);
        assert_eq!(tile.pixel_at(4, 6), Pixel::Color3);
        assert_eq!(tile.pixel_at(7, 7), Pixel::Color0);
    }

    fn smiley_face() -> [u8; 16] {
        [
            0b00000000, 0b00000000, 0b00100000, 0b00000100, 0b00100000, 0b00000100, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00100100, 0b00100100, 0b00011000, 0b00011000,
            0b00000000, 0b00000000,
        ]
    }
}
