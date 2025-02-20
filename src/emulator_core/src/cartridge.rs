mod header;
mod mbc1;
mod no_mbc;
mod mbc3;

use header::CartridgeType;
pub use header::Header;
pub use mbc1::MBC1;
use mbc3::MBC3;
pub use no_mbc::NoMBC;

#[cfg_attr(test, mockall::automock)]
pub trait Cartridge: Send {
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
        CartridgeType::MBC3 => Box::new(MBC3::new(rom))
    }
}
