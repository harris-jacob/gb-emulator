use std::time::SystemTime;

use rtc::{LatchedClockData, RTC};

use super::*;

/// MBC3 Cartridge. Supports up to 2 MiB ROM and 32 KiB RAM
/// Also features an RTC timer
pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_and_rtc_enabled: bool,
    // Represents whether a RAM bank or RTC register is available at standard
    // RAM address range.
    register_select: RegisterSelect,
    header: Header,
    rtc: RTC,
    persister: Option<Box<dyn CartridgePersistence>>,
}

enum RegisterSelect {
    // A ram bank was selected by writing 1-3 to 4000-5FFF
    RamBank(u8),
    // The seconds register of the RTC was selcted by writing 08 to 4000-5FFF
    RTCSeconds,
    // The minues register of the RTC was selcted by writing 09 to 4000-5FFF
    RTCMinutes,
    // The hours register of the RTC was selcted by writing 0A to 4000-5FFF
    RTCHours,
    // The days_lower of the RTC was selcted by writing 0B to 4000-5FFF
    RTCDaysLower,
    // The days_upper of the RTC was selcted by writing 0C to 4000-5FFF
    RTCDaysUpper,
}

impl Cartridge for MBC3 {
    /// Update the RTCs internal state relative to the current time.
    fn step(&mut self, _cycles: u8) {
        let now = SystemTime::now();
        self.rtc.update(now);
    }
    /// Read RAM at address range 0xA000-0xBFFF, range access depends on which ram bank is selected
    /// If ram is not enabled, 0xFF is returned. Panics if address is out of range
    fn read_ram(&self, address: u16) -> u8 {
        self.check_ram_range(address);

        match self.ram_and_rtc_enabled {
            // This is actually undefined behavior, the docs say that often
            // open bus is returned, often 0xFF
            false => 0xFF,
            true => match self.register_select {
                RegisterSelect::RamBank(ram_bank) => {
                    let remapped_address = (address - 0xA000) + (ram_bank as u16 * 0x2000);
                    self.ram[remapped_address as usize]
                }
                RegisterSelect::RTCSeconds => self.rtc.read_seconds(),
                RegisterSelect::RTCMinutes => self.rtc.read_minutes(),
                RegisterSelect::RTCHours => self.rtc.read_hours(),
                RegisterSelect::RTCDaysLower => self.rtc.read_days_lower(),
                RegisterSelect::RTCDaysUpper => self.rtc.read_days_upper(),
            },
        }
    }

    /// Write to RAM at address range 0xA000-0xBFFF, to the selected ram bank
    /// Panics if address is out of range
    fn write_ram(&mut self, address: u16, value: u8) {
        self.check_ram_range(address);
        if self.ram_and_rtc_enabled {
            match self.register_select {
                RegisterSelect::RamBank(ram_bank) => {
                    let remapped_address = (address - 0xA000) + (ram_bank as u16 * 0x2000);
                    self.ram[remapped_address as usize] = value;
                }
                RegisterSelect::RTCSeconds => self.rtc.write_seconds(SystemTime::now(), value),
                RegisterSelect::RTCMinutes => self.rtc.write_minutes(SystemTime::now(), value),
                RegisterSelect::RTCHours => self.rtc.write_hours(SystemTime::now(), value),
                RegisterSelect::RTCDaysLower => self.rtc.write_days_lower(SystemTime::now(), value),
                RegisterSelect::RTCDaysUpper => self.rtc.write_days_upper(SystemTime::now(), value),
            }
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
            // Enable reading and writing to RAM/RTC registers if the lower nibble is 0x0A
            0x0000..=0x1FFF => {
                self.ram_and_rtc_enabled = (value & 0x0F) == 0x0A;
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
            // Not confirmed but i've assumed here that writing anything else
            // does nothing.
            0x4000..=0x5FFF => match value {
                0x0..=0x3 => self.register_select = RegisterSelect::RamBank(value),
                0x8 => self.register_select = RegisterSelect::RTCSeconds,
                0x9 => self.register_select = RegisterSelect::RTCMinutes,
                0xA => self.register_select = RegisterSelect::RTCHours,
                0xB => self.register_select = RegisterSelect::RTCDaysLower,
                0xC => self.register_select = RegisterSelect::RTCDaysUpper,
                _ => {}
            },

            0x6000..=0x7FFF => {
                self.rtc.latch();
            }

            _ => panic!("Invalid address for MBC3: {:#06x}", address),
        }
    }

    fn save(&mut self) {
        if let Some(persister) = &mut self.persister {
            persister.write_ram(&self.ram);
        }
    }
}

impl MBC3 {
    pub fn new(rom: Vec<u8>, mut persister: Option<Box<dyn CartridgePersistence>>) -> Self {
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

        let ram = persister
            .as_mut()
            .and_then(|saver| {
                let ram = saver.load_ram();

                // TODO: this isn't great its also not covered in mbc1
                if ram.len() == 8000 * ram_banks {
                    Some(ram)
                } else {
                    None
                }
            })
            .unwrap_or(vec![0; 8000 * ram_banks]);

        MBC3 {
            rom,
            ram,
            rom_bank: 1,
            register_select: RegisterSelect::RamBank(0),
            ram_and_rtc_enabled: false,
            header,
            rtc: RTC::new(SystemTime::now()),
            persister,
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
                mbc3.register_select = RegisterSelect::RamBank(i);
                mbc3.write_ram(0xA000, i);
            }

            // read from each bank
            for i in 0..3 {
                mbc3.register_select = RegisterSelect::RamBank(i);
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

        MBC3::new(rom, None)
    }
}
