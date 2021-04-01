/* OP29 - ADD 16 */
static void OP_29(cpu_t* cpu) {
    cpu->reg->hl = alu_add16(cpu->reg, cpu->reg->hl, cpu->reg->hl);
}

/* OP2A - LD A (HL+) */
static void OP_2A(cpu_t* cpu) {
    cpu->reg->a = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->hl++;
}

/* DEC */
static void OP_25(cpu_t* cpu) {
    cpu->reg->h = alu_dec8(cpu->reg, cpu->reg->h);
}

/* INC */
static void OP_24(cpu_t* cpu) {
    cpu->reg->h = alu_inc8(cpu->reg, cpu->reg->h);
}

/* LD  d8*/
static void OP_2E(cpu_t* cpu, uint8_t val) {
    cpu->reg->l = val; 
}
