#include "register.h"


reg_t* reg_create() {
	reg_t* registers = (reg_t*)malloc(sizeof(reg_t));
    return registers;
}

void reg_destroy(reg_t** registers) {
	free(*registers);
	*registers = NULL;
}


void set_carry(reg_t* reg) {
	reg->f |= 1 << 4;
}

void set_halfcarry(reg_t* reg) {
	reg->f |= 1 << 5;
}

void set_zero(reg_t* reg) {
	reg->f |= 1 << 7;
}

void set_subtract(reg_t* reg) {
	reg->f |= 1 << 6;
}

void reset_carry(reg_t* reg) {
reg->f &= ~(1 << 4);
}

void reset_halfcarry(reg_t* reg) {
	reg->f &= ~(1 << 5);
}

void reset_zero(reg_t* reg) {
	reg->f &= ~(1 << 7);
}

void reset_subtract(reg_t* reg) {
	reg->f &= ~(1 << 6);
}

uint8_t get_carry(reg_t* reg) {
	uint8_t val = reg->f;
	return ((1 << 4) & val) >> 4;
}

uint8_t get_halfcarry(reg_t* reg) {
	uint8_t val = reg->f;
	return ((1 << 5) & val) >> 5;
}

uint8_t get_zero(reg_t* reg) {
	uint8_t val = reg->f;
	return ((1 << 7) & val) >> 7;
}

uint8_t get_subtract(reg_t* reg) {
	uint8_t val = reg->f;
	return ((1 << 6) & val) >> 6;
}