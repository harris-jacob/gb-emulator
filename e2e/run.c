#include "cpu.h"
#include "rom.h"
#include "memory.h"
#include "unistd.h"

int main(int argc, char *argv[]) {
    //"./build/e2e/06-ld r,r.gb"
    mmu_t* mmu = mmu_create();
    rom_t* rom = load_rom( "./build/e2e/06-ld r,r.gb",mmu);
    cpu_t* cpu = cpu_create(mmu);
    cpu->debug = true;

    // set all register values
    cpu_reset(cpu);


    for(; ;) {
        
        // step cpu
        cpu_step(cpu);


        // blarggs test - serial output
        if (mmu->addr[0xff02]) {
            char c = mmu->addr[0xff01];
            printf("%c", c);
            mmu->addr[0xff02] = 0x0;
            break;
        }

    }
}