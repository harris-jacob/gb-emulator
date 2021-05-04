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
    uint16_t clock_cycle;
    /* display debug logs */
    bool debug;
    /* IME: Interrupt Master Enable Flag */
    bool ime;
    /* is GB halted */
    bool halted;

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
void cpu_step(cpu_t* cpu);

/* Handle Opcode */
void cpu_handle_op(cpu_t* cpu, uint8_t opcode);

/* Handle unknown opcode */
void unknown_opcode(cpu_t* cpu);


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
    void (*execute)(void);
    /* number of cpu ticks */
    uint8_t ticks;
} extern const extended_ops[256];


#endif