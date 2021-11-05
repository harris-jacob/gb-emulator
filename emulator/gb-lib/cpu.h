#ifndef CPU_H_
#define CPU_H_
#include "mmu.h"
#include "register.h"


typedef struct cpu_t_ {
    /* CPU registers */
    reg_t* reg;
    /* Memory Unit */
    mmu_t* mmu;
    /* Current Clock cycle */
    uint32_t clock_cycle;
    /* display debug logs */
    bool debug;
    /* IME: Interrupt Master Enable Flag */
    bool ime;
    /* is GB halted */
    bool halted;
    /* is CPU stopped */
    bool stopped;

} cpu_t;

/* Create a CPU instance and initialize memory, reg etc. */
cpu_t* cpu_create(mmu_t* mmu);

/* Destroy a CPU instance */
void cpu_destroy(cpu_t** cpu);

/* Push data onto the stack and adjust the SP */
void stack_push(cpu_t* cpu, uint16_t data);

/* Pop 16 bits off bottom of stack, return data and adjust SP */
uint16_t stack_pop(cpu_t* cpu);

/* Reset the cpu's registers, clock and memory */
void cpu_reset(cpu_t* cpu);

/* Single step of the CPU */
uint32_t cpu_step(cpu_t* cpu);

/* Return name of an opcode */
char* get_op_name(uint8_t op);

/* Return the operand of an opcode */
uint16_t get_op_operand(cpu_t* cpu, uint8_t op);

/* get the opcode the PC is currently at */
uint16_t get_current_opcode(cpu_t* cpu);

/* Handle Opcode */
void cpu_handle_op(cpu_t* cpu, uint8_t opcode);

/* Handle unknown opcode */
void unknown_opcode(cpu_t* cpu);

/* check if any interrupts are enabled and if any have been registered. If so we need to handle them*/
void handle_interrupts(cpu_t* cpu);

/* Log operations to the console if the CPU is in debug mode */
void op_debug_log(cpu_t* cpu, uint8_t op); 

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
static void interrupt_handle(cpu_t* cpu, uint8_t i);

typedef struct op_details_t_ {
    /* name of eht operations */
    char* name;
    /* operand size */
    uint8_t operand_size;
    /* operand */
    uint16_t operand;
} op_details;

/* Operation container stolen from https://github.com/CTurt/Cinoop/blob/master/include/cpu.h */
struct op_t_ {
    /* Name of the operation */
    char* name;
    /* length of the operand */
    uint8_t operand_size;
    /* Func to execute */
    void* execute;
    /* number of cpu ticks */
    uint8_t ticks;
} extern const ops[256];

/* extended operation container */
struct extended_op_t_ {
    /* Name of the operation */
    char* name;
    /* Func to execute */
    void (*execute)(cpu_t*);
    /* number of cpu ticks */
    uint8_t ticks;
} extern const extended_ops[256];


#endif