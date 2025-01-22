pub struct PPU {
    ly: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self { ly: 0 }
    }

    pub fn read(&self, addr: u16) -> u8 {}

    pub fn write(&mut self, addr: u16, value: u8) {}
}
