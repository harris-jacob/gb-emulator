use std::usize;

use super::*;

/// MBC3 Cartridge. Supports up to 2 MiB ROM and 32 KiB RAM
/// Also features an RTC timer
pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    header: Header,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RTCRegisterSelect {
    RTC,
    RAM,
}

impl Cartridge for MBC3 {
    /// Read RAM at address range 0xA000-0xBFFF, range access depends on which ram bank is selected
    /// If ram is not enabled, 0xFF is returned. Panics if address is out of range
    fn read_ram(&self, address: u16) -> u8 {
        self.check_ram_range(address);

        // TODO: RTC select
        match self.ram_enabled {
            // This is actually undefined behavior, the docs say that often
            // open bus is returned, often 0xFF
            false => 0xFF,
            true => {
                let remapped_address = (address - 0xA000) + (self.ram_bank as u16 * 0x2000);
                self.ram[remapped_address as usize]
            }
        }
    }

    /// Write to RAM at address range 0xA000-0xBFFF, to the selected ram bank
    /// Panics if address is out of range
    /// TODO: battery backed RAM will need to be saved to disk
    fn write_ram(&mut self, address: u16, value: u8) {
        self.check_ram_range(address);
        if self.ram_enabled {
            let remapped_address = (address - 0xA000) + (self.ram_bank as u16 * 0x2000);
            self.ram[remapped_address as usize] = value;
        }
    }

    /// Read ROM at address range 0x0000-0x7FFF. For addresses in range 0x0000-0x3FFF, the
    /// address is used directly. For addresses in range 0x4000-0x7FFF, the accessed address
    /// depends on the selected ROM bank. Panics if address is out of range
    fn read_rom(&self, address: u16) -> u8 {
        self.check_rom_range(address);
        let remapped_address = match address {
            0x0000..=0x3FFF => address as usize,
            0x4000..=0x7FFF => {
                let offset = self.rom_bank as usize * 0x4000;
                offset + (address as usize - 0x4000)
            }
            _ => panic!("Invalid address for MBC3: {:#06x}", address),
        };

        *self.rom.get(remapped_address).unwrap_or(&0xff)
    }

    /// Writing to ROM, doesn't actually write to ROM, instead the MBC
    /// interprets writes to ROM memory address as control registers
    /// which alter the state of the MBC unit. The address still
    /// must be in the range 0x0000-0x7FFF or this function will panic
    /// The mappings are as follows:
    /// - 0x0000-0x1FFF: RAM enable/disable (lower nibble must set to 0x0A to enable)
    fn write_rom(&mut self, address: u16, value: u8) {
        self.check_rom_range(address);
        match address {
            // Enable ram if the lower nibble is 0x0A
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            // ROM bank selected using 7 bit value
            // If the ROM Bank Number is set to a higher value
            // than the number of banks in the cart, the bank number
            // is masked to the required number of bits.
            // e.g. a 256 KiB cart only needs a 4-bit bank number to
            // address all of its 16 banks, so this register is masked
            // to 4 bits. The upper bit would be ignored for bank selection.
            0x2000..=0x3FFF => {
                let nibble = value & self.rom_bank_mask();

                if value == 0 {
                    self.rom_bank = 1;
                } else {
                    self.rom_bank = nibble;
                }
            }

            // Writes of 0..3 maps the corresponding external RAM bank into memory
            // writing 08..0c maps the corresponding RTC register into memory.
            0x4000..=0x5FFF => {
                let new_value = self.ram_bank & 0b11;

                // If RAM is not large enough, setting this does nothing.
                if new_value as usize > self.header.rom_bank_count() {
                    return;
                }

                self.ram_bank = new_value;
            }

            // TODO: latch clock data
            0x6000..=0x7FFF => {}

            _ => panic!("Invalid address for MBC3: {:#06x}", address),
        }
    }
}

impl MBC3 {
    pub fn new(rom: Vec<u8>) -> Self {
        let header = Header::new(&rom);

        let ram_banks = header.ram_bank_count();
        let rom_banks = header.rom_bank_count();

        if rom_banks > 128 {
            panic!(
                "MBC3 only supports up to 128 ROM banks, found {}",
                rom_banks
            );
        }

        if ram_banks > 4 {
            panic!("MBC3 only supports up to 4 RAM banks, found {}", ram_banks);
        }

        let ram = vec![0; 8000 * ram_banks];

        MBC3 {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            header,
        }
    }

    /// In cases where the ROM bank number is higher than the number of banks
    /// the bank number is masked to min number of bits required to represent
    /// all banks. e.g. if there are 8 banks the bit mask is 0b111.
    fn rom_bank_mask(&self) -> u8 {
        match self.header.rom_size {
            header::ROMSize::KB32 => 0b1,
            header::ROMSize::KB64 => 0b11,
            header::ROMSize::KB128 => 0b111,
            header::ROMSize::KB256 => 0b1111,
            header::ROMSize::KB512 => 0b11111,
            header::ROMSize::KB1024 => 0b111111,
            header::ROMSize::KB2048 => 0b1111111,
            header::ROMSize::KB4096 => 0b11111111,
            header::ROMSize::KB8192 => 0b11111111,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod ram {
        use super::*;

        #[test]
        #[should_panic]
        fn read_ram_panics_if_address_out_of_range() {
            let mbc3 = mock_mbc3();

            mbc3.read_ram(0x0000);
        }

        #[test]
        #[should_panic]
        fn write_ram_panics_if_address_out_of_range() {
            let mut mbc3 = mock_mbc3();

            mbc3.write_ram(0x0000, 0);
        }

        #[test]
        fn read_ram_ram_disabled() {
            let mbc3 = mock_mbc3();

            assert_eq!(mbc3.read_ram(0xA000), 0xFF);
        }

        #[test]
        fn write_ram_ram_disabled() {
            let mut mbc3 = mock_mbc3();

            mbc3.write_ram(0xA000, 0x42);

            // Enable RAM
            mbc3.write_rom(0x0000, 0x0A);

            assert_eq!(mbc3.read_ram(0xA000), 0);
        }

        #[test]
        fn can_read_and_write_ram_banks_when_enabled() {
            let mut mbc3 = mock_mbc3();

            // Enable RAM
            mbc3.write_rom(0x0000, 0x0A);

            // write to each bank
            for i in 0..3 {
                mbc3.ram_bank = i;
                mbc3.write_ram(0xA000, i);
            }

            // read from each bank
            for i in 0..3 {
                mbc3.ram_bank = i;
                assert_eq!(mbc3.read_ram(0xA000), i);
            }
        }
    }

    mod rom {
        use super::*;

        #[test]
        #[should_panic]
        fn read_rom_panics_if_address_out_of_range() {
            let mbc3 = mock_mbc3();

            mbc3.read_rom(0x8000);
        }

        #[test]
        fn read_static_rom_memory() {
            let mut mbc3 = mock_mbc3();
            mbc3.rom[0x0000] = 0x42;

            assert_eq!(mbc3.read_rom(0x0000), 0x42);
        }

        #[test]
        fn read_rom_banked_memory() {
            let mut mbc3 = mock_mbc3();

            // Write known values to each bank
            for i in 1..4 {
                mbc3.rom[i * 0x4000] = i as u8;
            }

            for i in 1..4 {
                // Switch to ith bank
                mbc3.write_rom(0x2000, i as u8);
                assert_eq!(mbc3.read_rom(0x4000), i as u8);
            }
        }

        #[test]
        fn change_rom_bank_to_zero() {
            let mut mbc3 = mock_mbc3();

            // The first rom bank is fixed so trying to set the banked rom to 0
            // sets it to 1.
            mbc3.write_rom(0x2000, 0x00);
            assert_eq!(mbc3.rom_bank, 1);
        }

        #[test]
        fn change_rom_bank_to_value_larger_than_banks_available() {
            // If the ROM Bank Number is set to a higher value
            // than the number of banks in the cart, the bank number
            // is masked to the required number of bits.
            let mut mbc3 = mock_mbc3();
            mbc3.header.rom_size = header::ROMSize::KB64;
            mbc3.write_rom(0x2000, 0b11111111);
            assert_eq!(mbc3.rom_bank, 3);

            mbc3.header.rom_size = header::ROMSize::KB32;
            mbc3.write_rom(0x2000, 0b11111111);
            assert_eq!(mbc3.rom_bank, 1);
        }
    }
    fn mock_mbc3() -> MBC3 {
        let mut rom = vec![0; 0x8000 * 128];
        rom[0x147] = 0x01;
        rom[0x148] = 0x06;
        rom[0x149] = 0x03;

        MBC3::new(rom)
    }
}
