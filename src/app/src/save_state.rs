use std::fs::File;
use std::io::prelude::*;

use emulator_core::{CartridgeSaver, RTCState};

pub struct SaveState {
    rom_filepath: String,
    rtc_filepath: String,
}

impl SaveState {
    pub fn new(rom_name: &str) -> Self {
        Self {
            rom_filepath: rom_name.to_string() + ".sav",
            rtc_filepath: "rtc.sav".to_string(),
        }
    }
}

impl CartridgeSaver for SaveState {
    fn load_ram(&mut self) -> Vec<u8> {
        let mut ram = Vec::new();
        let mut file = File::open(&self.rom_filepath).expect("Creates file");
        file.read_to_end(&mut ram).expect("Write to file");

        ram
    }

    fn write_ram(&mut self, ram: &[u8]) {
        let mut file = File::create(&self.rom_filepath).expect("Creates file");
        file.write_all(ram).expect("Should write");
    }

    fn load_rtc(&mut self) -> RTCState {
        let mut contents = [0u8; 8];
        let mut file = File::create(&self.rtc_filepath).expect("Creates file");
        file.read_exact(&mut contents).expect("reads file");
        let zero = u64::from_be_bytes(contents);

        RTCState { zero }
    }

    fn write_rtc(&mut self, state: RTCState) {
        todo!()
    }
}
