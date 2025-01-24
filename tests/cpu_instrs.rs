use std::{fs::File, io::Read};

#[test]
fn passes_test_roms() {
    let filename = "./roms/cpu_instrs/cpu_instrs.gb";
    let mut fp = File::open(filename).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    let cartridge = emulator::create_cartridge(data);
    let ppu = emulator::PPU::new(Box::new(TestRenderer));
    let mmu = emulator::MMU::new(ppu, cartridge);

    let mut cpu = emulator::CPU::new(mmu);

    while cpu.clock < 60000000 {
        cpu.step();
    }

    let s = String::from_iter(cpu.mmu.serial);
    assert_eq!(s, "cpu_instrs\n\n01:ok  02:ok  03:ok  04:ok  05:ok  06:ok  07:ok  08:ok  09:ok  10:ok  11:ok  \n\nPassed all tests\n")
}

pub struct TestRenderer;

impl emulator::Renderer for TestRenderer {
    fn render(&mut self, _: [emulator::Pixel; 160 * 144]) {}
}
