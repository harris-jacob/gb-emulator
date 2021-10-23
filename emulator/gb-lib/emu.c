#include "emu.h"

emu_t* emu_create() {
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

    return emu;
}