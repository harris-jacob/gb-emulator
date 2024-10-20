mod core;

fn main() {
    let rom = core::ROM::from_disk("./roms/cpu_instrs/individual/04-op r,imm.gb").unwrap();
    // let rom = core::ROM::from_disk("./roms/cpu_instrs/individual/03-op sp,hl.gb").unwrap();
    // let rom = core::ROM::from_disk("./roms/cpu_instrs/individual/02-interrupts.gb").unwrap();
    // let rom = core::ROM::from_disk("./roms/cpu_instrs/individual/01-special.gb").unwrap();
    let mut cpu = core::CPU::new();
    cpu.load_rom(rom);

    let mut cycles = 0;
    cpu.debug_output();

    while cycles < 5000000 {
        cycles += cpu.step() as u64;
        if cycles != 0 {
            cpu.debug_output();
        }

        cycles += cpu.interrupt_step() as u64;
    }
}
