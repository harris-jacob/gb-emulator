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

/* OP00 - NOP */
static void OP_00(cpu_t* cpu) {
    return;
}

/* OP01 - ld bc d16 */
static void OP_01(cpu_t* cpu, uint16_t val) {
    cpu->reg->bc = val;
}

/* OP02 - ld (bc),a */
static void OP_02(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->bc, cpu->reg->a);
}

/* OP03 - INC BC */
static void OP_03(cpu_t* cpu) {
    cpu->reg->bc = alu_add16(cpu->reg, cpu->reg->bc, 1);
}

/* OP04 - INC B */
static void OP_04(cpu_t* cpu) {
    cpu->reg->b = alu_add8(cpu->reg, cpu->reg->b, 1);
}

/* OP05 - DECB */
static void OP_05(cpu_t* cpu) {
    cpu->reg->b = alu_subtract8(cpu->reg, cpu->reg->b, 1);
}


/* OP06 - LD, B, d8 */
static void OP_06(cpu_t* cpu, uint8_t val) {
    cpu->reg->b = val;
}

/* OP07 - RLCA */
static void OP_07(cpu_t* cpu, uint8_t val) {
    // TODO
}

/* OP08 - LD (a16), SP  */
static void OP_08(cpu_t* cpu, uint16_t addr) {
    mmu_write_addr16(cpu->mmu, addr, cpu->reg->sp);
}


/* OP09 - ADD HL, BC */
static void OP_09(cpu_t* cpu) {
    cpu->reg->hl = alu_add16(cpu->reg, cpu->reg->hl, cpu->reg->bc);
}

/* OP0A - LD A,(BC) */
static void OP_0A(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->bc);
    cpu->reg->a = val;
}

/* OP0B - DEC BC */
static void OP_0B(cpu_t* cpu) {
   cpu->reg->bc = alu_subtract16(cpu->reg, cpu->reg->bc, 1);
}

/* OP0C - INC C */
static void OP_0C(cpu_t* cpu) {
    cpu->reg->c = alu_add8(cpu->reg, cpu->reg->c, 1);
}

/* OP0D - DEC C */
static void OP_0D(cpu_t* cpu) {
    cpu->reg->c = alu_subtract8(cpu->reg, cpu->reg->c, 1);
}

/* OP0E - LD, C, d8 */
static void OP_0E(cpu_t* cpu, uint8_t val) {
    cpu->reg->c = val;
}

/* OP0F - LD, C, d8 */
static void OP_0F(cpu_t* cpu, uint8_t val) {
    // TODO
}

/* OP10 - LD, C, d8 */
static void OP_10(cpu_t* cpu, uint8_t val) {

}




