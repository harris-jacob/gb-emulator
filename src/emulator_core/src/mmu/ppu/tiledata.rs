use super::*;

/// Tiledata lives at 8000x97FF, each tile is 16 bytes. This means there is
/// room for 384 tiles.
///
/// Tiles are generally split into 3 blocks:
/// -- Block 0: $8000-87FF (128 tiles)
/// -- Block 1: $8800-8FFF (128 tiles)
/// -- Block 2: $9000-97FF (128 tiles)
pub struct TileData {
    data: [u8; 0x1800],
}

/// There are two ways of accessing tile data:
/// - SignedMethod ($8000 method). Which uses 0x8000 as the base pointer using unsigned addressing.
///   Meaning we can access tiles 0-255. i.e. Blocks 0 and 1.
/// - UnsignedMethod ($8800 method). Uses 0x8800 as the base pointer. And a signed offset
///   which means we can access tiles 128-394 i.e. Blocks 1 and 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileAddressingMethod {
    Signed,
    Unsigned,
}

impl<'a> TileData {
    pub fn new() -> Self {
        Self { data: [0; 0x1800] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        Self::check_addr_range(addr);

        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        Self::check_addr_range(addr);

        self.data[addr as usize] = value;
    }

    pub(super) fn tile_at(
        &'a self,
        tile_number: u8,
        addressing_method: TileAddressingMethod,
    ) -> BackgroundTile<'a> {
        let start = match addressing_method {
            TileAddressingMethod::Signed => {
                let tile_number = tile_number as i8;
                (0x1000 + tile_number as i16 * 16) as usize
            }

            TileAddressingMethod::Unsigned => tile_number as usize * 16,
        };

        BackgroundTile::new(&self.data[start..start + 16])
    }

    pub(super) fn sprite_tile_at(
        &'a self,
        tile_number: u8,
        sprite_size: SpriteSize,
    ) -> SpriteTile<'a> {
        let tile_number = {
            match sprite_size {
                SpriteSize::Normal => tile_number,
                // For "long" sprites, the tile number of the 'top' tile is
                // the requested tile number with the least significant bit
                // set to 0, the 'bottom' tile is the requested tile with the
                // least significant bit set to 1. Here are are calculating the
                // tile number of the top tile, knowing that below when we setup
                // our tile, the next tile in the array will become the bottom
                // tile.
                SpriteSize::Long => tile_number & !0b1,
            }
        };

        let start = tile_number as usize * 16;

        // The sprites can either be 8x8 like backgrounds, meaning the can
        // be encoded into 16 bytes (2bpp) or they can be 16x8, meaning they
        // can be encoded into 32 bytes. This ensures the slice is the correct
        // size based on the sprite height. (16/32 bytes).
        let end = start + (sprite_size.height() as usize) * 2;

        SpriteTile::new(&self.data[start..end], sprite_size)
    }

    fn check_addr_range(addr: u16) {
        if addr > 0x18FF {
            panic!("Address out of range for TileData access")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn panics_reading_out_of_range_addr() {
        let tiledata = TileData::new();

        tiledata.read(0x5000);
    }

    #[should_panic]
    #[test]
    fn panics_writing_out_of_range_addr() {
        let mut tiledata = TileData::new();

        tiledata.write(0x5000, 10);
    }

    #[test]
    fn read_write_data() {
        let mut tiledata = TileData::new();

        for addr in 0..0x1800 {
            tiledata.write(addr, 1);
            assert_eq!(tiledata.read(addr), 1);
        }
    }

    mod tile_at {
        use super::*;

        #[test]
        fn unsigned_access() {
            let smiley_face = smiley_face();
            let mut tiledata = TileData::new();

            // put a smiley face at tile 100
            for i in 0..16 {
                tiledata.write(100 * 16 + i, smiley_face[i as usize]);
            }

            let tile = tiledata.tile_at(100, TileAddressingMethod::Unsigned);

            assert_smiley_face_bg_tile(tile);
        }

        #[test]
        fn signed_access_positive_offset() {
            let smiley_face = smiley_face();
            let mut tiledata = TileData::new();

            // put a smiley face at tile 300
            for i in 0..16 {
                tiledata.write(300 * 16 + i, smiley_face[i as usize]);
            }

            let tile = tiledata.tile_at(44, TileAddressingMethod::Signed);

            assert_smiley_face_bg_tile(tile);
        }

        #[test]
        fn signed_access_negative_offset() {
            let smiley_face = smiley_face();
            let mut tiledata = TileData::new();

            // put a smiley face at tile 300
            for i in 0..16 {
                tiledata.write(200 * 16 + i, smiley_face[i as usize]);
            }

            let offset = -56;
            let tile = tiledata.tile_at(offset as u8, TileAddressingMethod::Signed);

            assert_smiley_face_bg_tile(tile);
        }
    }

    mod sprite_tile_at {
        use super::*;

        #[test]
        fn normal_size_tile() {
            let smiley_face = smiley_face();
            let mut tiledata = TileData::new();

            // put a smiley face at tile 100
            for i in 0..16 {
                tiledata.write(100 * 16 + i, smiley_face[i as usize]);
            }

            let tile = tiledata.sprite_tile_at(100, SpriteSize::Normal);
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
        fn long_size_tile() {
            let smiley_face = tall_smile_face();
            let mut tiledata = TileData::new();

            // put a smiley face at tile 100
            for i in 0..32 {
                tiledata.write(100 * 16 + i, smiley_face[i as usize]);
            }

            let tile = tiledata.sprite_tile_at(100, SpriteSize::Long);
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
    }

    fn assert_smiley_face_bg_tile(tile: BackgroundTile) {
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

    fn tall_smile_face() -> [u8; 32] {
        [
            0, 0, 32, 4, 32, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 36,
            24, 24, 0, 0,
        ]
    }
}
