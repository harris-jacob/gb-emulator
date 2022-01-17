#include "cpu.h"
#include "rom.h"
#include "ppu.h"
#include "memory.h"
#include "emu.h"
#include <emscripten.h>

#include <stdio.h>

emu_t* emu;

EMSCRIPTEN_KEEPALIVE
void init() {
    // set all register values
    emu = emu_create(true);
    emu_load_rom(emu, "roms/02-interrupts.gb");
}

EMSCRIPTEN_KEEPALIVE
reg_t* get_reg() {
    return emu->cpu->reg;
}

EMSCRIPTEN_KEEPALIVE 
void step() {
    emu_step(emu);
}

EMSCRIPTEN_KEEPALIVE
uint8_t* get_rom() {
    return emu->mmu->rom[0];
}

EMSCRIPTEN_KEEPALIVE
char* get_next_op_name() {
    uint8_t opcode = peek_next_opcode(emu->cpu);
    return get_op_name(opcode);
}

EMSCRIPTEN_KEEPALIVE
uint8_t get_next_opcode() {
    return peek_next_opcode(emu->cpu);
}

EMSCRIPTEN_KEEPALIVE
uint16_t  get_next_op_operand() {
    return get_op_operand(emu->cpu, emu->cpu->reg->pc+1);
}

EMSCRIPTEN_KEEPALIVE
uint16_t get_operand_by_pc(uint16_t pc) {
    return get_op_operand(emu->cpu, pc);
}

EMSCRIPTEN_KEEPALIVE
uint8_t get_opcode_by_pc(uint16_t pc) {
    return mmu_read_addr8(emu->cpu->mmu, pc);
}

EMSCRIPTEN_KEEPALIVE
char* get_op_name_by_pc(uint16_t pc) {
    uint8_t opcode = get_opcode_by_pc(pc);
    return get_op_name(opcode);
}

EMSCRIPTEN_KEEPALIVE
uint8_t get_opcode_operand_size(uint8_t opcode) {
    return ops[opcode].operand_size;
}