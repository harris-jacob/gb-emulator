#include "cpu.h"
#include "register.h"
#include "mmu.h"


cpu_t* cpu_create() {
    reg_t* reg = reg_create();
    mmu_t* mmu = mmu_create();
    cpu_t cpu;

    cpu.mmu = mmu;
    cpu.reg = reg;

    cpu.debug = false;
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

void cpu_reset(cpu_t* cpu) {
    // Reset the registers
    cpu->reg->a = 0x01;
    cpu->reg->f = 0;
    cpu->reg->b = 0;
    cpu->reg->c = 0x13;
    cpu->reg->d = 0;
    cpu->reg->e = 0xd8;
    cpu->reg->h = 0x01;
    cpu->reg->l = 0x4d;
    cpu->reg->sp = 0xfffe;
    cpu->reg->pc = 0x100;

    // reset memory
    mmu_destroy(cpu->mmu);
    cpu->mmu = mmu_create();

    // Reset interrupts
    mmu_disable_all_interrupts(cpu->mmu);
}

void cpu_step(cpu_t* cpu) {
    uint8_t opcode = mmu_read_addr8(cpu->mmu, cpu->reg->pc++);

    cpu_handle_op(cpu, opcode);
}