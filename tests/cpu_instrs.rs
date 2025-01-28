use std::{fs::File, io::Read, sync::Arc};

#[test]
fn passes_test_roms() {
    let filename = "./roms/cpu_instrs/cpu_instrs.gb";
    let mut fp = File::open(filename).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    let cartridge = emulator::create_cartridge(data);
    let ppu = emulator::PPU::new(Arc::new(TestRenderer));
    let mmu = emulator::MMU::new(ppu, cartridge);

    let mut clock = 0;
    let mut cpu = emulator::CPU::new(mmu);

    while clock < 60000000 {
        let cycles = cpu.step();
        clock += cycles as u32;
    }

    let s = String::from_iter(cpu.mmu.serial);
    assert_eq!(s, "cpu_instrs\n\n01:ok  02:ok  03:ok  04:ok  05:ok  06:ok  07:ok  08:ok  09:ok  10:ok  11:ok  \n\nPassed all tests\n")
}

pub struct TestRenderer;

impl emulator::Renderer for TestRenderer {
    fn render(&self, _: [u32; 160 * 144]) {}
}
