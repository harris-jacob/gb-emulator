use super::Pixel;
use super::SpriteFlags;
use super::SpriteSize;

/// A Sprite tile, is similar to a background tile but supports the additional
/// options afforded tiles. For example, background tiles are always 8x8 but
/// SpriteTiles can be rendered in tall mode, which two 8x8 tiles are stacked
/// on top of one another to create a single 16x8 sprite. Also Sprite tiles
/// can be flipped in x or y.
#[derive(Debug)]
pub struct SpriteTile<'a>(&'a [u8]);

impl<'a> SpriteTile<'a> {
    pub fn new(data: &'a [u8]) -> SpriteTile<'a> {
        Self(data)
    }

    pub fn pixel_at(&self, x: u8, y: u8, size: SpriteSize, flags: SpriteFlags) -> Pixel {
        Self::check_coord_range(x, y, size);

        match size {
            SpriteSize::Normal => self.pixel_at_normal_sprite(x, y, flags),
            SpriteSize::Long => todo!(),
        }
    }

    fn pixel_at_normal_sprite(&self, mut x: u8, mut y: u8, flags: SpriteFlags) -> Pixel {
        if flags.x_flip() {
            x = 7 - x
        }

        if flags.y_flip() {
            y = 7 - y
        }

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

    fn check_coord_range(x: u8, y: u8, size: SpriteSize) {
        match size {
            SpriteSize::Normal => {
                if y >= 8 || x >= 8 {
                    panic!("pixel out of range")
                }
            }
            SpriteSize::Long => {
                if y >= 16 || x >= 8 {
                    panic!("pixel out of range")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod normal_sprite {
        use super::*;
        #[should_panic]
        #[test]
        fn tile_at_panics_reading_out_of_range_coords() {
            let tile = SpriteTile::new(&[0; 16]);

            tile.pixel_at(9, 9, SpriteSize::Normal, SpriteFlags::new(0));
        }

        #[test]
        fn pixel_at_default_flags() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face);
            let flags = SpriteFlags::new(0);
            let size = SpriteSize::Normal;

            assert_eq!(tile.pixel_at(2, 1, size, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 2, size, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 1, size, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 2, size, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 5, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 5, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 6, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 6, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(7, 7, size, flags), Pixel::Color0);
        }

        #[test]
        fn pixel_with_x_flip() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face);
            let flags = SpriteFlags::new(0b00100000);
            let size = SpriteSize::Normal;

            assert_eq!(tile.pixel_at(2, 1, size, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 2, size, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 1, size, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 2, size, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 5, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 5, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 6, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 6, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(7, 7, size, flags), Pixel::Color0);
        }

        #[test]
        fn pixel_with_y_flip() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face);
            let flags = SpriteFlags::new(0b01000000);
            let size = SpriteSize::Normal;

            assert_eq!(tile.pixel_at(2, 5, size, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 6, size, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 5, size, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 6, size, flags), Pixel::Color2);

            assert_eq!(tile.pixel_at(2, 2, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 2, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 1, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 1, size, flags), Pixel::Color3);

            assert_eq!(tile.pixel_at(7, 7, size, flags), Pixel::Color0);
        }

        #[test]
        fn pixel_with_both_flipped() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face);
            let flags = SpriteFlags::new(0b01100000);
            let size = SpriteSize::Normal;

            assert_eq!(tile.pixel_at(2, 5, size, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 6, size, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 5, size, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 6, size, flags), Pixel::Color1);

            assert_eq!(tile.pixel_at(2, 2, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 2, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 1, size, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 1, size, flags), Pixel::Color3);

            assert_eq!(tile.pixel_at(7, 7, size, flags), Pixel::Color0);
        }
    }

    mod long_sprite {
        use super::*;
        #[should_panic]
        #[test]
        fn tile_at_panics_reading_out_of_range_coords_tall_sprite() {
            let tile = SpriteTile::new(&[0; 32]);

            tile.pixel_at(17, 7, SpriteSize::Long, SpriteFlags::new(0));
        }
    }

    fn smiley_face() -> [u8; 16] {
        [
            0b00000000, 0b00000000, 0b00100000, 0b00000100, 0b00100000, 0b00000100, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00100100, 0b00100100, 0b00011000, 0b00011000,
            0b00000000, 0b00000000,
        ]
    }
}
