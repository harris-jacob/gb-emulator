#include "cpu.h"
#include "register.h"
#include "mmu.h"


cpu_t* cpu_create() {
    reg_t* reg = reg_create();
    mmu_t* mmu = mmu_create();
    cpu_t cpu;

    cpu.mmu = mmu;
    cpu.reg = reg;
}

void cpu_destroy(cpu_t	**cpu) {
	free(*cpu);
	*cpu = NULL;
}

void stack_push(cpu_t *cpu, uint16_t val) {
    cpu->reg->sp-=2;
   mmu_write_addr16(cpu->mmu, cpu->reg->sp, val);
}

uint16_t stack_pop(cpu_t* cpu) {
    uint16_t val = mmu_read_addr16(cpu->mmu, cpu->reg->sp);
    cpu->reg->sp+=2;

    return val;
}
