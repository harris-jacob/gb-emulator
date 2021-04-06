#include "mmu.h"
#include "register.h"


typedef struct cpu_t_ {
    /* CPU registers */
    reg_t* reg;
    /* Memory Unit */
    mmu_t* mmu;
    /* Current Clock cycle */
    uint16_t clock_cycle;
} cpu_t;

/* Create a CPU instance */
cpu_t* cpu_create();

/* Destroy a CPU instance */
void cpu_destroy(cpu_t** cpu);

/* Push data onto the stack and adjust the SP */
void stack_push(cpu_t* cpu, uint16_t data);

/* Pop 16 bits off bottom of stack, return data and adjust SP */
uint16_t stack_pop(cpu_t* cpu);










