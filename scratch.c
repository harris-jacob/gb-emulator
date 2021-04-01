/* OP29 - ADD HL HL */
static void OP_29(cpu_t* cpu) {
    cpu->reg->hl = alu_add16(cpu->reg, cpu->reg->hl, cpu->reg->hl);
}

/* OP2A - LD A (HL+) */
static void OP_2A(cpu_t* cpu) {
    cpu->reg->a = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->hl++;
}