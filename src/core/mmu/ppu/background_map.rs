/// There are two of these in memory at $9000-$9BFF & $9C00-9FFF. Each represents
/// a 32x32 map, where each entry in the grid corresponds to a tile number.
/// These maps control which tiles are displayed in the background / window layers.
pub struct BackgroundMap([u8; 0x3FF]);

impl BackgroundMap {
    pub fn new() -> Self {
        Self([0; 0x3FF])
    }

    pub fn read(&self, addr: u16) -> u8 {
        Self::check_addr_range(addr);

        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        Self::check_addr_range(addr);

        self.0[addr as usize] = value;
    }

    pub fn check_addr_range(addr: u16) {
        if addr > 0x3FF {
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

        map.read(0x400);
    }

    #[test]
    #[should_panic]
    fn out_of_range_write_panics() {
        let mut map = BackgroundMap::new();

        map.write(0x400, 0);
    }

    #[test]
    fn read_write() {
        let mut map = BackgroundMap::new();
        for i in 0u16..0x3FF {
            map.write(i, 1);

            assert_eq!(map.read(i), 1);
        }
    }
}
