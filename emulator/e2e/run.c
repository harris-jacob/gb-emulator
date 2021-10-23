#include "cpu.h"
#include "rom.h"
#include "ppu.h"
#include "memory.h"
#include "unistd.h"

int main(int argc, char *argv[]) {
    //"./build/e2e/06-ld r,r.gb"
    mmu_t* mmu = mmu_create();
    rom_t* rom = load_rom( "./gb-test-roms/cpu_instrs/individual/02-interrupts.gb",mmu);
    cpu_t* cpu = cpu_create(mmu);
    char out[100];
    uint8_t i=0;
    // set all register values
    cpu_reset(cpu);
    cpu->debug = false;

    for(; ;) {
        // step cpu
        uint32_t clock_delta = cpu_step(cpu);
        ppu_clock_step(cpu->mmu, cpu->clock_cycle);

        // blarggs test - serial output
        if(mmu->addr[0xff02] == 0x81) {
            out[i++] = mmu->addr[0xff01];
            printf("%c", mmu->addr[0xff01]);
            mmu->addr[0xff02] = 0;
        }
    }
}