use std::{fs::File, io::Read, sync::Arc};

use emulator_core::*;

pub trait BlarggTestCase {
    /// Path to the test ROM
    fn filepath() -> String;

    /// Expected Serial port output of the test ROM
    fn expected_output() -> String;

    /// Number of CPU cycles to execute before completing test case
    fn steps() -> u32;

    fn run() {
        let mut cpu = setup_emulator(&Self::filepath());
        let mut clock = 0;

        while clock < Self::steps() {
            let cycles = cpu.step();
            clock += cycles as u32;
        }

        let s = String::from_iter(cpu.mmu.serial);

        assert_eq!(s, Self::expected_output());
    }
}

pub trait MooneyeTestCase {
    /// Path to the test ROM
    fn filepath() -> String;

    /// Number of CPU cycles to execute before completing test case
    fn steps() -> u32;

    fn run() {
        let mut cpu = setup_emulator(&Self::filepath());
        let mut clock = 0;

        while clock < Self::steps() {
            let cycles = cpu.step();
            clock += cycles as u32;
        }

        let registers = cpu.registers();

        assert_eq!(registers.b, 3);
        assert_eq!(registers.c, 5);
        assert_eq!(registers.d, 8);
        assert_eq!(registers.e, 13);
        assert_eq!(registers.h, 21);
        assert_eq!(registers.l, 34);
    }
}

pub fn setup_emulator(rom_path: &str) -> CPU {
    let mut fp = File::open(rom_path).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    let cartridge = create_cartridge(data);
    let ppu = PPU::new(Arc::new(TestRenderer));
    let joypad = Arc::new(Joypad::new());
    let mmu = MMU::new(ppu, cartridge, joypad);

    CPU::new(mmu)
}

pub struct TestRenderer;

impl Renderer for TestRenderer {
    fn render(&self, _: [u32; 160 * 144]) {}
}
