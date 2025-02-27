mod header;
mod mbc1;
mod mbc3;
mod no_mbc;
mod rtc;

use header::CartridgeType;
pub use header::Header;
use mbc1::MBC1;
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

    /// Save the state of RAM
    fn save(&mut self) {}

    // Update is called every emulation cycle and provides cartridges with a
    // way to update their state cyclicly. The default impl of this method does
    // nothing because most cartridges have no cyclic functionality.
    // The primary exception is cartridge types that contain RTCs (real time
    // clocks), RTC registers must be updated every step of the emulation.
    fn step(&mut self, _ticks: u8) {}

    fn check_ram_range(&self, address: u16) {
        if !(0xA000..=0xBFFF).contains(&address) {
            panic!("Invalid address for MBC1 RAM: {:#06x}", address);
        }
    }

    fn check_rom_range(&self, address: u16) {
        if address > 0x7FFF {
            panic!("Invalid address for MBC1 ROM: {:#06x}", address);
        }
    }
}

/// Exposes persistence logic for the Cartridge, used to initialise the
/// cartridge state when it is loaded and to save its state when ejected.
pub trait CartridgePersistence: Send {
    // Called when trying to initialise battery backed RAM.
    // The size of the output RAM is important, it should
    // be exactly the same size as the array provided in the `write_ram`
    // call. If the size of the Vec is incorrect, the emulator will ignore
    // this and start from a blank RAM state.
    fn load_ram(&mut self) -> Vec<u8>;
    // Called when trying to save battery backed RAM.
    fn write_ram(&mut self, ram: &[u8]);
}

pub fn create_cartridge(
    rom: Vec<u8>,
    persistance: Box<dyn CartridgePersistence>,
) -> Box<dyn Cartridge> {
    let header = Header::new(&rom);

    match header.cartridge_type {
        CartridgeType::ROMOnly => Box::new(NoMBC::new(rom)),
        CartridgeType::MBC1 => Box::new(MBC1::new(rom, None)),
        CartridgeType::MBC1Battery => Box::new(MBC1::new(rom, Some(persistance))),
        CartridgeType::MBC3 => Box::new(MBC3::new(rom, None)),
        CartridgeType::MBC3Battery => Box::new(MBC3::new(rom, Some(persistance))),
    }
}
