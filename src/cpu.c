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
    cpu->reg->af = 0x1B0;
    cpu->reg->bc = 0x13;
    cpu->reg->de = 0x0D8;
    cpu->reg->hl = 0x14D;
    cpu->reg->sp = 0xfffe;
    cpu->reg->pc = 0x100;

    // Reset interrupts
    mmu_disable_all_interrupts(cpu->mmu);
}

void cpu_step(cpu_t* cpu) {
    uint8_t opcode = mmu_read_addr8(cpu->mmu, cpu->reg->pc++);
    if(cpu->stopped) return;


    if(opcode ==0xC7) {
        printf("hello");
    }
    
    if(cpu->debug && opcode) {
        printf("PC at: 0x%x\n",cpu->reg->pc - 1);
        printf("SP at 0x%x\n", cpu->reg->sp);
    }
    cpu_handle_op(cpu, opcode);
}

void unknown_opcode(cpu_t* cpu) {
    uint8_t op = mmu_read_addr8(cpu->mmu, --cpu->reg->pc);
    char s[50];
    sprintf(s,"Tried to execute unknown opcode: 0x%x\n", op);
    printf("%s",s);
    printf("PC at: 0x%x\n", cpu->reg->pc);
}

void cpu_handle_op(cpu_t* cpu, uint8_t op) {

    // Print for debug
    if(cpu->debug && op) {
        printf("executing op number: %x  op name: %s \n", op, ops[op].name);
        printf("register values:  af=0x%x;  bc=0x%x;  de=0x%x;  hl=0x%x\n", 
            cpu->reg->af, cpu->reg->bc, cpu->reg->de, cpu->reg->hl);
    }

    if(cpu->reg->pc > 0x217) {
        printf("helo");
    }

    // Handle opcode
    switch (ops[op].operand_size)
    {
    case 0:
        ((void (*)(cpu_t*))ops[op].execute)(cpu);
        break;
        
    case 1: ;
        uint8_t operand_8 = mmu_read_addr8(cpu->mmu, cpu->reg->pc++);

        if(cpu->debug) {
            printf("OP called with value: 0x%x\n", operand_8);
        }

        ((void (*)(cpu_t*, u_int8_t))ops[op].execute)(cpu, operand_8);
        break;

    case 2: ; 
        uint16_t operand_16 = mmu_read_addr16(cpu->mmu, cpu->reg->pc);
        
        if(cpu->debug) {
            printf("OP called with value: 0x%x\n", operand_16);
        }
        
        cpu->reg->pc+=2;
        ((void (*)(cpu_t*, uint16_t))ops[op].execute)(cpu, operand_16);

        break;

    
    default:
        break;
    }

    if(cpu->reg->pc == 0) {
        printf("hello");
    }

    cpu->clock_cycle+= ops[op].ticks;
}