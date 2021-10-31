#include "cpu.h"
#include "rom.h"
#include "ppu.h"
#include "memory.h"
#include "unistd.h"

#include <stdio.h>

mmu_t* mmu;
//rom_t* rom = load_rom( "./gb-test-roms/cpu_instrs/individual/02-interrupts.gb",mmu);
cpu_t* cpu;

int main(int argc, char *argv[]) {
    // set all register values
    mmu = mmu_create();
    cpu = cpu_create(mmu);
    cpu_reset(cpu);
}

reg_t* peek_reg() {
    return cpu->reg;
}

