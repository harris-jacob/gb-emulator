#include "cpu.h"
#include "rom.h"
#include "ppu.h"
#include "memory.h"
#include <emscripten.h>

#include <stdio.h>

mmu_t* mmu;
//rom_t* rom = load_rom( "./gb-test-roms/cpu_instrs/individual/02-interrupts.gb",mmu);
cpu_t* cpu;

EMSCRIPTEN_KEEPALIVE
void init() {
    // set all register values
    mmu = mmu_create();
    rom_t* rom = load_rom("roms/02-interrupts.gb", mmu);
    cpu = cpu_create(mmu);
    cpu_reset(cpu);
}

EMSCRIPTEN_KEEPALIVE
reg_t* get_reg() {
    return cpu->reg;
}

EMSCRIPTEN_KEEPALIVE 
void step() {
    uint32_t clock_delta = cpu_step(cpu);
    ppu_clock_step(cpu->mmu, cpu->clock_cycle);
}

EMSCRIPTEN_KEEPALIVE
uint8_t* get_rom() {
    return cpu->mmu->rom[0];
}