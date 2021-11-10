#include "emu.h"

emu_t* emu_create(bool debug) {
    // create a memory unit
    mmu_t* mmu = mmu_create();
    mmu_init(mmu);
    // create rom cartridge
    rom_t* rom = rom_create(mmu);
    // create CPU 
    cpu_t* cpu = cpu_create(mmu);
    cpu_reset(cpu);

    emu_t* emu = (emu_t*)malloc(sizeof(emu_t));
    emu->cpu = cpu;
    emu->mmu = mmu;
    emu->rom = rom;
    emu->debug = debug;
    return emu;
}

void emu_load_rom(emu_t* emu, const char* filepath) {
    emu->rom = load_rom(filepath, emu->mmu);
}

/** Single emulator step */
void emu_step(emu_t* emu) {
    uint32_t clock_delta = cpu_step(emu->cpu);
    ppu_clock_step(emu->mmu, emu->cpu->clock_cycle);
}