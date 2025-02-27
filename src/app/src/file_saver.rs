use std::fs;
use std::io::prelude::*;

use emulator_core::CartridgePersistence;

/// Save the state of the cartridge to a local file.
pub struct FileSaver {
    filepath: String,
}

impl FileSaver {
    pub fn new(rom_name: &str) -> Self {
        let filepath = "./roms/saves/".to_string() + rom_name + ".sav";
        Self { filepath }
    }
}

impl CartridgePersistence for FileSaver {
    fn load_ram(&mut self) -> Vec<u8> {
        fs::read(&self.filepath).expect("Should open file")
    }

    fn write_ram(&mut self, ram: &[u8]) {
        let mut file = fs::File::create(&self.filepath).expect("Creates file");
        file.write_all(ram).expect("Should write");
    }
}
