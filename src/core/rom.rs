use std::{
    error::Error,
    fs::File,
    io::{Read, Seek},
};

use super::*;

pub struct ROM {
    data: Vec<u8>,
    pub header: Header,
}

#[derive(Debug)]
pub struct Header {
    // 16 byte title of the rom
    pub name: String,
    // cartridge type TODO: enum
    pub cartridge_type: u8,
    // Enum for rom size TODO: enum
    pub rom_size: u8,
    // External ram for the cartridge TODO: enum
    pub ram_size: u8,
}

impl ROM {
    pub fn from_disk(filename: &str) -> Result<ROM, Box<dyn Error>> {
        let mut fp = File::open(filename)?;
        let mut data = Vec::new();

        let size = fp.read_to_end(&mut data)?;
        let header = ROM::read_header(&data);

        Ok(ROM { data, header })
    }
    pub fn load(&self, mmu: &mut MMU) {
        mmu.load_rom(&self.data);
    }

    fn read_header(data: &Vec<u8>) -> Header {
        Header {
            name: String::from_utf8_lossy(&data[0x134..0x143]).to_string(),
            cartridge_type: data[0x147],
            rom_size: data[0x148],
            ram_size: data[0x149],
        }
    }
}
