use super::Pixel;
use super::SpriteFlags;
use super::SpriteSize;

/// A Sprite tile, is similar to a background tile but supports the additional
/// options afforded tiles. For example, background tiles are always 8x8 but
/// SpriteTiles can be rendered in tall mode, which two 8x8 tiles are stacked
/// on top of one another to create a single 16x8 sprite. Also Sprite tiles
/// can be flipped in x or y.
#[derive(Debug)]
pub struct SpriteTile<'a> {
    data: &'a [u8],
    size: SpriteSize,
}

impl<'a> SpriteTile<'a> {
    pub fn new(data: &'a [u8], size: SpriteSize) -> SpriteTile<'a> {
        Self { data, size }
    }

    pub fn pixel_at(&self, mut x: u8, mut y: u8, flags: SpriteFlags) -> Pixel {
        if flags.x_flip() {
            x = 7 - x
        }

        if flags.y_flip() {
            y = self.size.height() - 1 - y
        }

        let line_first_byte = self.idx(y * 2);
        let line_second_byte = self.idx(y * 2 + 1);

        let bit_one = line_first_byte >> (7 - x) & 0b1;
        let bit_two = line_second_byte >> (7 - x) & 0b1;

        let pixel = (bit_two << 1) | bit_one;

        pixel.into()
    }

    fn idx(&self, idx: u8) -> u8 {
        self.data[idx as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod normal_sprite {
        use super::*;
        #[test]
        fn pixel_at_default_flags() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face, SpriteSize::Normal);
            let flags = SpriteFlags::new(0);

            assert_eq!(tile.pixel_at(2, 1, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 1, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 5, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 5, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 6, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 6, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(7, 7, flags), Pixel::Color0);
        }

        #[test]
        fn pixel_with_x_flip() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face, SpriteSize::Normal);
            let flags = SpriteFlags::new(0b00100000);

            assert_eq!(tile.pixel_at(2, 1, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 1, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 5, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 5, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 6, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 6, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(7, 7, flags), Pixel::Color0);
        }

        #[test]
        fn pixel_with_y_flip() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face, SpriteSize::Normal);
            let flags = SpriteFlags::new(0b01000000);

            assert_eq!(tile.pixel_at(2, 5, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 6, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 5, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 6, flags), Pixel::Color2);

            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 1, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 1, flags), Pixel::Color3);

            assert_eq!(tile.pixel_at(7, 7, flags), Pixel::Color0);
        }

        #[test]
        fn pixel_at_with_both_flipped() {
            let face = smiley_face();
            let tile = SpriteTile::new(&face, SpriteSize::Normal);
            let flags = SpriteFlags::new(0b01100000);

            assert_eq!(tile.pixel_at(2, 5, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 6, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 5, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 6, flags), Pixel::Color1);

            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 1, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 1, flags), Pixel::Color3);

            assert_eq!(tile.pixel_at(7, 7, flags), Pixel::Color0);
        }
    }

    mod long_sprite {
        use super::*;

        #[test]
        fn pixel_at_default_flags() {
            let face = tall_smile_face();
            let tile = SpriteTile::new(&face, SpriteSize::Long);
            let flags = SpriteFlags::new(0);

            assert_eq!(tile.pixel_at(2, 1, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 1, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color2);

            assert_eq!(tile.pixel_at(2, 13, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 13, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 14, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 14, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(7, 15, flags), Pixel::Color0);
        }

        #[test]
        fn pixel_at_x_flip() {
            let face = tall_smile_face();
            let tile = SpriteTile::new(&face, SpriteSize::Long);
            let flags = SpriteFlags::new(0b00100000);

            assert_eq!(tile.pixel_at(2, 1, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 1, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color1);

            assert_eq!(tile.pixel_at(2, 13, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 13, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 14, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 14, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(7, 15, flags), Pixel::Color0);

        }

        #[test]
        fn pixel_at_y_flip() {
            let face = tall_smile_face();
            let tile = SpriteTile::new(&face, SpriteSize::Long);
            let flags = SpriteFlags::new(0b01000000);

            assert_eq!(tile.pixel_at(2, 14, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(2, 13, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 14, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 13, flags), Pixel::Color2);

            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 1, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 1, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(7, 15, flags), Pixel::Color0);

        }        

        #[test]
        fn pixel_at_with_both_flipped() {
            let face = tall_smile_face();
            let tile = SpriteTile::new(&face, SpriteSize::Long);
            let flags = SpriteFlags::new(0b01100000);

            assert_eq!(tile.pixel_at(2, 14, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(2, 13, flags), Pixel::Color2);
            assert_eq!(tile.pixel_at(5, 14, flags), Pixel::Color1);
            assert_eq!(tile.pixel_at(5, 13, flags), Pixel::Color1);

            assert_eq!(tile.pixel_at(2, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(5, 2, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(3, 1, flags), Pixel::Color3);
            assert_eq!(tile.pixel_at(4, 1, flags), Pixel::Color3);

            assert_eq!(tile.pixel_at(7, 15, flags), Pixel::Color0);
        }

    }

    fn smiley_face() -> [u8; 16] {
        [
            0b00000000, 0b00000000, 0b00100000, 0b00000100, 0b00100000, 0b00000100, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00100100, 0b00100100, 0b00011000, 0b00011000,
            0b00000000, 0b00000000,
        ]
    }

    fn tall_smile_face() -> [u8; 32] {
        [
            0, 0, 32, 4, 32, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 36,
            24, 24, 0, 0,
        ]
    }
}
