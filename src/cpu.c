#include "cpu.h"
#include "register.h"


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

/* OP00 - NOP */
static void OP_00(cpu_t* cpu) {
    return;
}

/* OP01 - ld bc d16 */
static void OP_01(cpu_t* cpu, uint16_t val) {
    cpu->reg->bc = val;
}

/* OP02 - ld (bc),a */
static void OP_02(cpu_t* cpu, uint16_t val) {
    cpu->reg->bc = cpu->reg->a;
}

/* OP03 - ld (),a */
static void OP_03(cpu_t* cpu) {
    cpu->reg->bc++;
}

/* OP04 -   */