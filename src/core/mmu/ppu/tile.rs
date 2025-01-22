/// A tile is a 8x8 pixel image encoded using the 2bpp representation.
/// This type handles the nitty-gritty of the encoding and allows access to
/// the pixel color information at given x,y co-ordinates.
pub struct Tile([u8; 16]);

#[derive(Debug, PartialEq, Eq)]
pub enum Pixel {
    Color0,
    Color1,
    Color2,
    Color3,
}

impl Tile {
    pub fn new(data: [u8; 16]) -> Self {
        Self(data)
    }

    /// Get the color of the pixel at the given x,y coordinate.
    /// Since tiles are 8x8 this function panics if y|x are
    /// larger than 7.
    pub fn pixel_at(&self, x: u8, y: u8) -> Pixel {
        Self::check_coord_range(x, y);

        let line_first_byte = self.idx(y * 2);
        let line_second_byte = self.idx(y * 2 + 1);

        let pixel = (line_first_byte >> (7 - x) & 0b1) | ((line_second_byte >> (7 - x) & 0b1) << 1);

        match pixel {
            0 => Pixel::Color0,
            1 => Pixel::Color1,
            2 => Pixel::Color2,
            3 => Pixel::Color3,
            _ => unreachable!(),
        }
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
        let tile = Tile::new([0; 16]);

        tile.pixel_at(9, 9);
    }

    #[test]
    fn pixel_at_returns_correct_pixel() {
        let tile = Tile::new(smiley_face());

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
