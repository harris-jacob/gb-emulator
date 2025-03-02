use super::*;

/// MBC1 Cartridge. Supports up to 2 MiB ROM and 32 KiB RAM
pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    banking_mode: BankingMode,
    header: Header,
    // Used to persist state
    persister: Option<Box<dyn CartridgePersistence>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BankingMode {
    Rom,
    Ram,
}

impl Cartridge for MBC1 {
    /// Read RAM at address range 0xA000-0xBFFF, range access depends on which ram bank is selected
    /// If ram is not enabled, 0xFF is returned. Panics if address is out of range
    fn read_ram(&self, address: u16) -> u8 {
        self.check_ram_range(address);

        match (self.banking_mode, self.ram_enabled) {
            // This is actually undefined behavior, the docs say that often
            // open bus is returned, often 0xFF
            (_, false) => 0xFF,
            (BankingMode::Ram, true) => {
                let remapped_address = (address - 0xA000) + (self.ram_bank as u16 * 0x2000);
                self.ram[remapped_address as usize]
            }
            (BankingMode::Rom, true) => {
                // Always access the first RAM bank in ROM banking mode
                let remapped_address = address - 0xA000;
                self.ram[remapped_address as usize]
            }
        }
    }

    /// Write to RAM at address range 0xA000-0xBFFF, to the selected ram bank
    /// Panics if address is out of range
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
            _ => panic!("Invalid address for MBC1: {:#06x}", address),
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
            // Unlike MBC1 all 7 bits of the ROM bank number are written
            // directly to the address
            0x2000..=0x3FFF => {
                let nibble = value & self.rom_bank_mask();

                if value & 0b11111 == 0 {
                    self.rom_bank = 1;
                } else {
                    self.rom_bank = nibble;
                }
            }

            // Either this register is used to select a RAM bank, or
            // as an additional 2 bits of the ROM bank number (for 1Mib
            // ROM or larger carts)
            0x4000..=0x5FFF => {
                match self.banking_mode {
                    // Used to set bits 5 & 6 of ROM bank
                    BankingMode::Rom => {
                        let new_value = self.rom_bank | (value & 0b11) << 5;
                        self.rom_bank = new_value & self.rom_bank_mask();
                    }
                    BankingMode::Ram => {
                        let new_value = self.ram_bank & 0b11;

                        // If RAM is not large enough, setting this does nothing.
                        if new_value as usize > self.header.rom_bank_count() {
                            return;
                        }

                        self.ram_bank = new_value;
                    }
                }
            }

            // Banking mode select
            0x6000..=0x7FFF => {
                self.banking_mode = match value & 0x01 {
                    0 => BankingMode::Rom,
                    1 => BankingMode::Ram,
                    _ => unreachable!(),
                };
            }

            _ => panic!("Invalid address for MBC1: {:#06x}", address),
        }
    }

    fn save(&mut self) {
        if let Some(persister) = &mut self.persister {
            persister.write_ram(&self.ram);
        }
    }
}

impl MBC1 {
    pub fn new(rom: Vec<u8>, mut persister: Option<Box<dyn CartridgePersistence>>) -> Self {
        let header = Header::new(&rom);

        let ram_banks = header.ram_bank_count();
        let rom_banks = header.rom_bank_count();

        validate_rom_bank_size(rom_banks);
        validate_ram_bank_size(ram_banks);

        let ram = persister.as_mut().map(|persister| persister.load_ram());

        let ram = valid_ram(ram, ram_banks);

        MBC1 {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            banking_mode: BankingMode::Rom,
            header,
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

/// Panic if the number of ROM banks in the header exceeds the number supported
/// by MBC1.
fn validate_rom_bank_size(rom_bank_count: usize) {
    if rom_bank_count <= 128 {
        return;
    }

    panic!(
        "MBC1 only supports up to 128 ROM banks, found {}",
        rom_bank_count
    );
}

/// Panic if hte number of RAM banks in the header exceeds the number supported
/// by MBC1
fn validate_ram_bank_size(ram_bank_count: usize) {
    if ram_bank_count <= 4 {
        return;
    }
    panic!(
        "MBC1 only supports up to 4 RAM banks, found {}",
        ram_bank_count
    );
}

/// When we load RAM from disk we need to ensure its the size we expect, otherwise
/// we run the risk of out of bounds access etc. So, if we find our loaded ram
/// to be 'corrupted' we just ignore it and create a new empty RAM vec.
fn valid_ram(suspect_ram: Option<Vec<u8>>, ram_banks: usize) -> Vec<u8> {
    match suspect_ram {
        Some(suspect_ram) if suspect_ram.len() == 8000 * ram_banks => suspect_ram,
        _ => vec![0; 8000 * ram_banks],
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
            let mbc1 = mock_mbc1();

            mbc1.read_ram(0x0000);
        }

        #[test]
        #[should_panic]
        fn write_ram_panics_if_address_out_of_range() {
            let mut mbc1 = mock_mbc1();

            mbc1.write_ram(0x0000, 0);
        }

        #[test]
        fn read_ram_ram_disabled() {
            let mbc1 = mock_mbc1();

            assert_eq!(mbc1.read_ram(0xA000), 0xFF);
        }

        #[test]
        fn write_ram_ram_disabled() {
            let mut mbc1 = mock_mbc1();

            mbc1.write_ram(0xA000, 0x42);

            // Enable RAM
            mbc1.write_rom(0x0000, 0x0A);

            assert_eq!(mbc1.read_ram(0xA000), 0);
        }

        #[test]
        fn can_read_and_write_ram_banks_in_ram_banking_mode() {
            let mut mbc1 = mock_mbc1();

            // Enable RAM
            mbc1.write_rom(0x0000, 0x0A);

            // Switch to RAM banking mode
            mbc1.write_rom(0x6000, 1);

            // write to each bank
            for i in 0..3 {
                mbc1.ram_bank = i;
                mbc1.write_ram(0xA000, i);
            }

            // read from each bank
            for i in 0..3 {
                mbc1.ram_bank = i;
                assert_eq!(mbc1.read_ram(0xA000), i);
            }
        }

        #[test]
        fn only_first_ram_bank_available_in_rom_banking_mode() {
            let mut mbc1 = mock_mbc1();
            mbc1.ram = vec![0; 0x2000 * 4];

            // Enable RAM
            mbc1.write_rom(0x0000, 0x0A);

            // Switch to RAM banking mode
            mbc1.write_rom(0x6000, 0);

            // write to each bank
            for i in 0..3 {
                mbc1.ram_bank = i;
                mbc1.write_ram(0xA000, i);
            }

            //Switch to ROM banking mode
            mbc1.write_rom(0x6000, 0);

            // read from each bank
            for i in 0..3 {
                mbc1.ram_bank = i;
                assert_eq!(mbc1.read_ram(0xA000), 0);
            }
        }

        #[test]
        fn ignores_writes_to_ram_banks_out_of_range() {
            let mut rom = vec![0; 0x8000 * 128];
            rom[0x147] = 0x01;
            rom[0x148] = 0x06;
            // only KB2 of ram
            rom[0x149] = 0x01;
            let mut mbc1 = MBC1::new(rom, None);

            mbc1.ram = vec![0; 0x2000 * 4];

            // Enable RAM
            mbc1.write_rom(0x0000, 0x0A);

            // Switch to RAM banking mode
            mbc1.write_rom(0x6000, 0);

            mbc1.write_rom(0x4000, 3);

            assert_eq!(mbc1.ram_bank, 0);
        }
    }

    mod rom {
        use super::*;

        #[test]
        #[should_panic]
        fn read_rom_panics_if_address_out_of_range() {
            let mbc1 = mock_mbc1();

            mbc1.read_rom(0x8000);
        }

        #[test]
        fn read_static_rom_memory() {
            let mut mbc1 = mock_mbc1();
            mbc1.rom[0x0000] = 0x42;

            assert_eq!(mbc1.read_rom(0x0000), 0x42);
        }

        #[test]
        fn read_rom_banked_memory() {
            let mut mbc1 = mock_mbc1();

            // Write known values to each bank
            for i in 1..4 {
                mbc1.rom[i * 0x4000] = i as u8;
            }

            for i in 1..4 {
                // Switch to ith bank
                mbc1.write_rom(0x2000, i as u8);
                assert_eq!(mbc1.read_rom(0x4000), i as u8);
            }
        }

        #[test]
        fn change_rom_bank_to_zero() {
            let mut mbc1 = mock_mbc1();

            // The first rom bank is fixed so trying to set the banked rom to 0
            // sets it to 1.
            mbc1.write_rom(0x2000, 0x00);
            assert_eq!(mbc1.rom_bank, 1);
        }

        #[test]
        fn change_rom_bank_to_value_larger_than_banks_available() {
            // If the ROM Bank Number is set to a higher value
            // than the number of banks in the cart, the bank number
            // is masked to the required number of bits.
            let mut mbc1 = mock_mbc1();
            mbc1.header.rom_size = header::ROMSize::KB64;
            mbc1.write_rom(0x2000, 0b11111111);
            assert_eq!(mbc1.rom_bank, 3);

            mbc1.header.rom_size = header::ROMSize::KB32;
            mbc1.write_rom(0x2000, 0b11111111);
            assert_eq!(mbc1.rom_bank, 1);
        }

        #[test]
        fn change_rom_bank_to_zero_edge_case() {
            let mut mbc1 = mock_mbc1();
            mbc1.header.rom_size = header::ROMSize::KB64;

            // Even with smaller ROMs that use less than 5 bits for bank selection,
            // the full 5-bit register is still compared for the bank 00→01 translation
            // logic. As a result if the ROM is 256 KiB or smaller, it is possible to
            // map bank 0 to the 4000–7FFF region — by setting the 5th bit to 1 it will
            // prevent the 00→01 translation (which looks at the full 5-bit register,
            // and sees the value $10, not $00), while the bits actually used for bank
            // selection (4, in this example) are all 0, so bank $00 is selected.
            mbc1.write_rom(0x2000, 0b10000);
            assert_eq!(mbc1.rom_bank, 0);
        }

        #[test]
        fn in_rom_banking_mode_higher_banks_are_available() {
            let mut mbc1 = mock_mbc1();
            mbc1.rom = vec![0; 0x4000 * 130];
            mbc1.rom[0x4000 * 0b1111111] = 0x42;

            // Switch to ROM banking mode
            mbc1.write_rom(0x6000, 0);

            mbc1.write_rom(0x2000, 0b11111);
            mbc1.write_rom(0x4000, 0b11);
            assert_eq!(mbc1.read_rom(0x4000), 0x42);
        }

        #[test]
        fn in_ram_banking_mode_higher_bits_are_ignored() {
            let mut mbc1 = mock_mbc1();
            mbc1.rom = vec![0; 0x4000 * 130];

            // Switch to RAM banking mode
            mbc1.write_rom(0x6000, 1);

            mbc1.write_rom(0x2000, 0b11111);
            mbc1.write_rom(0x4000, 0b11);
            assert_eq!(mbc1.read_rom(0x4000), 0);
        }
    }
    fn mock_mbc1() -> MBC1 {
        let mut rom = vec![0; 0x8000 * 128];
        rom[0x147] = 0x01;
        rom[0x148] = 0x06;
        rom[0x149] = 0x03;

        MBC1::new(rom, None)
    }
}
