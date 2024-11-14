mod no_mbc;
mod mbc1;
mod header;

use header::CartridgeType;
pub use no_mbc::NoMBC;
pub use mbc1::MBC1;
pub use header::Header;

pub trait Cartridge {
    /// Read a byte from the cartridge's RAM
    fn read_ram(&self, address: u16) -> u8;
    /// Write a byte to the cartridge's RAM
    fn write_ram(&mut self, address: u16, value: u8);
    /// Read a byte from the cartridge's ROM
    fn read_rom(&self, address: u16) -> u8;
    /// Write a byte to the cartridge's ROM
    fn write_rom(&mut self, address: u16, value: u8);

    fn check_ram_range(&self, address: u16) {
        if address < 0xA000 || address > 0xBFFF {
            panic!("Invalid address for MBC1 RAM: {:#06x}", address);
        }
    }

    fn check_rom_range(&self, address: u16) {
        if address > 0x7FFF {
            panic!("Invalid address for MBC1 ROM: {:#06x}", address);
        }
    }
}

pub fn create_cartridge(rom: Vec<u8>) -> Box<dyn Cartridge> {
    let header = Header::new(&rom);

    match header.cartridge_type {
        CartridgeType::ROMOnly => Box::new(NoMBC::new(rom)),
        CartridgeType::MBC1 => Box::new(MBC1::new(rom)),
    }
}
