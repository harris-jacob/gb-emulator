/// There are two of these in memory at $9000-$9BFF & $9C00-9FFF. Each represents
/// a 32x32 map, where each entry in the grid corresponds to a tile number.
/// These maps control which tiles are displayed in the background / window layers.
pub struct BackgroundMap([u8; 0x400]);

/// Which background map is selected.
/// - Background map 0: $9800-9BFF
/// - Background map 1: $9C00-9FFF
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BGMapSelection {
    Map0,
    Map1,
}

impl BackgroundMap {
    pub fn new() -> Self {
        Self([0; 0x400])
    }

    pub fn read(&self, addr: u16) -> u8 {
        Self::check_addr_range(addr);

        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        Self::check_addr_range(addr);

        self.0[addr as usize] = value;
    }

    // Tile number in the backgrounc map at the specificed X, Y coordinate
    pub fn tile_number_at(&self, x: u8, y: u8) -> u8 {
        let x = x / 8;
        let y = y / 8;

        self.0[y as usize * 32 + x as usize]
    }

    pub fn check_addr_range(addr: u16) {
        if addr > 0x400 {
            panic!("address out of range for background map")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn out_of_range_read_panics() {
        let map = BackgroundMap::new();

        map.read(0x401);
    }

    #[test]
    #[should_panic]
    fn out_of_range_write_panics() {
        let mut map = BackgroundMap::new();

        map.write(0x401, 0);
    }

    #[test]
    fn read_write() {
        let mut map = BackgroundMap::new();
        for i in 0u16..0x400 {
            map.write(i, 1);

            assert_eq!(map.read(i), 1);
        }
    }

    #[test]
    fn tile_index_at() {
        let mut map = BackgroundMap::new();
        map.write(0, 1);
        map.write(1, 2);
        map.write(32, 3);
        map.write(33, 4);

        assert_eq!(map.tile_number_at(0, 0), 1);
        assert_eq!(map.tile_number_at(8, 3), 2);
        assert_eq!(map.tile_number_at(2, 8), 3);
        assert_eq!(map.tile_number_at(8, 8), 4);
    }
}
