use super::*;

/// Tiledata lives at 8000x97FF, each tile is 16 bytes. This means there is
/// room for 384 tiles.
///
/// Tiles are generally split into 3 blocks:
/// -- Block 0: $8000-87FF (128 tiles)
/// -- Block 1: $8800-8FFF (128 tiles)
/// -- Block 2: $9000-97FF (128 tiles)
pub struct TileData {
    data: [u8; 0x17FF],
}

/// There are two ways of accessing tile data:
/// - SignedMethod ($8000 method). Which uses 0x8000 as the base pointer using unsigned addressing.
///   Meaning we can access tiles 0-255. i.e. Blocks 0 and 1.
/// - UnsignedMethod ($8800 method). Uses 0x8800 as the base pointer. And a signed offset
///   which means we can access tiles 128-394 i.e. Blocks 1 and 2.
pub enum TileAddressingMethod {
    SignedAddressing,
    UnsignedAddressing,
}

impl TileData {
    pub fn new() -> Self {
        Self { data: [0; 0x17FF] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        Self::check_addr_range(addr);

        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        Self::check_addr_range(addr);

        self.data[addr as usize] = value;
    }

    pub(super) fn tile_at(&self, tile_number: u8, addressing_method: TileAddressingMethod) -> Tile {
        let start = match addressing_method {
            TileAddressingMethod::SignedAddressing => {
                let tile_number = tile_number as i8;

                (0x1000 + (tile_number as i8) as i16 * 16) as usize
            }

            TileAddressingMethod::UnsignedAddressing => tile_number as usize * 16,
        };

        Tile::new(self.tile_data(start))
    }

    // TODO: make this work without the extra array
    fn tile_data(&self, start: usize) -> [u8; 16] {
        let mut data = [0; 16];
        for i in 0..16 {
            data[i] = self.data[start + i];
        }

        data
    }

    fn check_addr_range(addr: u16) {
        if addr > 0x17FF {
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

        for addr in 0..0x17FF {
            tiledata.write(addr, 1);

            dbg!(128 >> 7);

            assert_eq!(tiledata.read(addr), 1);
        }
    }

    #[test]
    fn tile_at_unsigned_access() {
        let smiley_face = smiley_face();
        let mut tiledata = TileData::new();

        // put a smiley face at tile 100
        for i in 0..16 {
            tiledata.write(100 * 16 + i, smiley_face[i as usize]);
        }

        let tile = tiledata.tile_at(100, TileAddressingMethod::UnsignedAddressing);

        assert_smiley_face(tile);
    }

    #[test]
    fn tile_at_signed_access_positive_offset() {
        let smiley_face = smiley_face();
        let mut tiledata = TileData::new();

        // put a smiley face at tile 300
        for i in 0..16 {
            tiledata.write(300 * 16 + i, smiley_face[i as usize]);
        }

        let tile = tiledata.tile_at(44, TileAddressingMethod::SignedAddressing);

        assert_smiley_face(tile);
    }

    #[test]
    fn tile_at_signed_access_negative_offset() {
        let smiley_face = smiley_face();
        let mut tiledata = TileData::new();

        // put a smiley face at tile 300
        for i in 0..16 {
            tiledata.write(200 * 16 + i, smiley_face[i as usize]);
        }

        let offset = -56;
        let tile = tiledata.tile_at(offset as u8, TileAddressingMethod::SignedAddressing);

        assert_smiley_face(tile);
    }

    fn assert_smiley_face(tile: Tile) {
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
