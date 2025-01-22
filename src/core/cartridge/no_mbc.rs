use super::Cartridge;

/// 32 KiB ROM with 8KiB of RAM. No memory banking
pub struct NoMBC {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMBC {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram = vec![0; 8000];
        NoMBC { rom, ram }
    }
}

impl Cartridge for NoMBC {
    fn read_ram(&self, address: u16) -> u8 {
        self.check_ram_range(address);
        let address = address as usize - 0xA000;
        self.ram[address]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.check_ram_range(address);
        let address = address as usize - 0xA000;
        self.ram[address] = value;
    }

    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {}
}
