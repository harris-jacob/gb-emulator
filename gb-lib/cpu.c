#include "cpu.h"
#include "mmu.h"
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

    cpu->halted = false;
    cpu->ime = false;
    cpu->stopped = false;
}
uint16_t last_pc;
uint16_t last_op;
uint16_t old_sp;
/* execute the next instruction, update the cpu clock and return the number of cycles added */
uint32_t cpu_step(cpu_t* cpu) {
    uint32_t old_clock = cpu->clock_cycle;
    old_sp = cpu->reg->sp;

    if(cpu->stopped){
        return 0;
    }


    uint8_t opcode = mmu_read_addr8(cpu->mmu, cpu->reg->pc++);

    last_op = opcode;
    last_pc = cpu->reg->pc;

    // handle halt and interrupt
    if(cpu->halted || cpu->ime) {
       handle_interrupts(cpu);
    }
    
    if(cpu->debug && opcode) {
        printf("PC at: 0x%x\n",cpu->reg->pc - 1);
        printf("SP at 0x%x\n", cpu->reg->sp);
    }
    cpu_handle_op(cpu, opcode);

    return cpu->clock_cycle - old_clock;
}

void unknown_opcode(cpu_t* cpu) {
    uint8_t op = mmu_read_addr8(cpu->mmu, --cpu->reg->pc);
    char s[50];
    sprintf(s,"Tried to execute unknown opcode: 0x%x\n", op);
    printf("%s",s);
    printf("PC at: 0x%x\n", cpu->reg->pc);
}

/*
* if we come accross an interrupt we
* - unhalt and reset the interrupt master enable (ime)
* - reset the interrupt bit that was triggered
* - jump to the correct address, based on the interrupt that was triggered
*   here i is the bit number of the interrupt:
*   Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
*   Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
*   Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)
*   Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)
*   Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)
*/
static void interrupt_handle(cpu_t* cpu, uint8_t i) {
   // if just halt we can unhalt
   if(!cpu->ime && cpu->halted) {
       cpu->halted;
       return;
   }

   // Turn off IME
   cpu->ime = false;
   cpu->halted = false;

   stack_push(cpu, cpu->reg->pc);

   switch (i) {
       case 0:
        cpu->reg->pc = 0x40;
        reset_vblank(cpu->mmu);
        return;
       case 1:
        cpu->reg->pc = 0x48;
        reset_lcdstat(cpu->mmu);
        return;
       case 2:
        cpu->reg->pc = 0x50;
        reset_timer(cpu->mmu);
        return;
       case 3:
        cpu->reg->pc = 0x58;
        reset_serial(cpu->mmu);
        return;
       case 4:
        cpu->reg->pc = 0x60;
        reset_joypad(cpu->mmu);
        return;
        default:
            return;
   }
}

/* check if any interrupts are enabled and if any have been registered. If so we need to handle them*/
void handle_interrupts(cpu_t* cpu) {
    uint8_t interrupt = mmu_read_addr8(cpu->mmu, 0xFF0F);
    uint8_t enabled = mmu_read_addr8(cpu->mmu, 0xFFFF);

    if(!interrupt) return;
    // check interrupt and interrupt enable set
    for(int i=0; i<4; i++) {
        uint8_t int_bit = (interrupt>>i)&1;
        uint8_t enabled_bit = (enabled>>i)&1;

        if(int_bit && enabled_bit) {
            interrupt_handle(cpu, i);
        }
    }
}

void cpu_handle_op(cpu_t* cpu, uint8_t op) {

    // Print for debug
    if(cpu->debug && op) {
        printf("executing op number: %x  op name: %s \n", op, ops[op].name);
        printf("register values:  a=0x%x; b=0x%x;  c=0x%x;  d=0x%x;  e=0x%x;  f=0x%x;  h=0x%x;  l=0x%x;\n", 
            cpu->reg->a, cpu->reg->b, cpu->reg->c, cpu->reg->d, cpu->reg->e, cpu->reg->f, 
            cpu->reg->h, cpu->reg->l);
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

    cpu->clock_cycle+= ops[op].ticks;
}