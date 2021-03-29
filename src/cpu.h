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










