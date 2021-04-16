#include "cpu.h"
#include <stdlib.h>


cpu_t* cpu_create(mmu_t* mmu) {
    reg_t* reg = reg_create();
    cpu_t* cpu = (cpu_t*)(malloc(sizeof(cpu_t)));

    cpu->mmu = mmu;
    cpu->reg = reg;

    cpu->debug = false;

    return cpu;
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

    // Reset interrupts
    mmu_disable_all_interrupts(cpu->mmu);
}

void cpu_step(cpu_t* cpu) {
    uint8_t opcode = mmu_read_addr8(cpu->mmu, cpu->reg->pc++);

    cpu_handle_op(cpu, opcode);
}

void unknown_opcode(cpu_t* cpu) {
    uint8_t op = mmu_read_addr8(cpu->mmu, --cpu->reg->sp);
    char s[50];
    sprintf(s,"Tried to execute unknown opcode: 0x%03x.", op);
    printf("%s",s);
}

void cpu_handle_op(cpu_t* cpu, uint8_t op) {

    // Print for debug
    if(cpu->debug) {
        printf("%d \n %s", op, ops[op].name);
    }

    // Handle opcode
    switch (ops[op].operand_size)
    {
    case 0:
        ((void (*)(void))ops[op].execute)();
    case 1: ;
        uint8_t operand_8 = mmu_read_addr8(cpu->mmu, cpu->reg->sp++);
        ((void (*)(u_int8_t))ops[op].execute)(operand_8);
        break;

    case 2: ; 
        uint16_t operand_16 = mmu_read_addr16(cpu->mmu, cpu->reg->sp);
        cpu->reg->sp+=2;
        ((void (*)(uint16_t))ops[op].execute)(operand_16);
    
    default:
        break;
    }

    cpu->clock_cycle+= ops[op].ticks;
}