use std::{fs::File, io::Read};

mod core;

fn main() {
    let filename = "./roms/cpu_instrs/cpu_instrs.gb";
    // let filename = "./roms/cpu_instrs/individual/11-op a,(hl).gb";
    // let filename = ("./roms/cpu_instrs/individual/08-misc instrs.gb";
    // let filename = "./roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb";
    // let filename = "./roms/cpu_instrs/individual/06-ld r,r.gb";
    // let filename = "./roms/cpu_instrs/individual/05-op rp.gb";
    // let filename = "./roms/cpu_instrs/individual/04-op r,imm.gb";
    // let filename = "./roms/cpu_instrs/individual/03-op sp,hl.gb";
    // let filename = "./roms/cpu_instrs/individual/02-interrupts.gb";
    // let filename = "./roms/cpu_instrs/individual/01-special.gb";

    let mut fp = File::open(filename).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    let cartridge = core::create_cartridge(data);

    let mut cpu = core::CPU::new(cartridge);

    let mut cycles = 0;
    // cpu.debug_output();

    while cycles < 100000000 {
        cycles += cpu.step() as u64;
        // if cycles != 0 {
        //     cpu.debug_output();
        // }

        cycles += cpu.interrupt_step() as u64;
    }
}
