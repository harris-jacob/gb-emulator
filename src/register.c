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

bool should_add_halfcarry8(uint8_t a, uint8_t b) {	
	return ((a & 0xf) + (b & 0xf) & 0x10) == 0x10;
}

bool should_add_halfcarry16(uint16_t a, uint16_t b) {
	return ((a & 0xff) + (b & 0xff) & 0x100) == 0x100;
}

bool should_add_carry8(uint8_t a, uint8_t b) {
	return ((a & 0xff) + (b & 0xff) & 0x100) == 0x100;
}

bool should_add_carry16(uint16_t a, uint16_t b) {
	return ((a & 0xffff) + (b & 0xffff) & 0x10000) == 0x10000;
}

bool should_sub_halfcarry8(uint8_t a, uint8_t b) {	
	return ((a & 0xf) - (b & 0xf) & 0x10) == 0x10;
}

bool should_sub_halfcarry16(uint16_t a, uint16_t b) {
	return ((a & 0xff) - (b & 0xff) & 0x100) == 0x100;
}

bool should_sub_carry8(uint8_t a, uint8_t b) {
	return ((a & 0xff) - (b & 0xff) & 0x100) == 0x100;
}

bool should_sub_carry16(uint16_t a, uint16_t b) {
	return ((a & 0xffff) - (b & 0xffff) & 0x10000) == 0x10000;
}

uint8_t alu_add8(reg_t* reg, uint8_t a, uint8_t b) {
	reset_subtract(reg);

	if(should_add_carry8(a, b)) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(should_add_halfcarry8(a, b)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	uint8_t val = a + b;
	if (val == 0)
	{
		set_zero(reg);
	} else {
		reset_zero(reg);
	} 

	return val;
}

uint8_t alu_adc8(reg_t* reg, uint8_t a, uint8_t b) {
	reset_subtract(reg);

	uint8_t val = a + b + get_carry(reg);
	
	if(should_add_carry8(b, 1)) {
		set_carry(reg);
	} else {
		if(should_add_carry8(a, b+1)) {
			set_carry(reg);
		} else {
			reset_carry(reg);
		}
	}

	if(should_add_halfcarry8(b, 1)) {
		set_halfcarry(reg);
	} else {
		if(should_add_halfcarry8(a, b+1)) {
			set_halfcarry(reg);
		} else {
			reset_halfcarry(reg);
		}
	}

	return val;
}


uint8_t alu_sbc8(reg_t* reg, uint8_t a, uint8_t b) {
	set_subtract(reg);

	uint8_t val = a + b + get_carry(reg);
	
	if(should_sub_carry8(b, 1)) {
		set_carry(reg);
	} else {
		if(should_sub_carry8(a, b+1)) {
			set_carry(reg);
		} else {
			reset_carry(reg);
		}
	}

	if(should_sub_halfcarry8(b, 1)) {
		set_halfcarry(reg);
	} else {
		if(should_sub_halfcarry8(a, b+1)) {
			set_halfcarry(reg);
		} else {
			reset_halfcarry(reg);
		}
	}

	return val;
}

uint16_t alu_add16(reg_t* reg, uint16_t a, uint16_t b) {
	reset_subtract(reg);


	if(should_add_carry16(a, b)) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(should_add_halfcarry16(a, b)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	uint16_t val = a + b; 

	return val;
}

uint8_t alu_subtract8(reg_t* reg, uint8_t a, uint8_t b) {
	set_subtract(reg);

	if(should_sub_carry8(a, b)) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(should_sub_halfcarry8(a, b)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	uint8_t val = a - b;
	if (val == 0)
	{
		set_zero(reg);
	} else {
		reset_zero(reg);
	} 

	return val;
}


uint8_t alu_inc8(reg_t* reg, uint8_t a) {
	reset_subtract(reg);

	if(should_add_halfcarry8(a, 1)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	a++;

	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	return a;
}

uint8_t alu_dec8(reg_t* reg, uint8_t a) {
	set_subtract(reg);

	if(should_sub_halfcarry8(a, 1)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	a--;

	if(a == 0 ) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}
}

uint8_t rotate_r(uint8_t a, uint8_t n) {
	return ( a >> n ) | (a << (8-n));
}

uint8_t rotate_l(uint8_t a, uint8_t n) {
	return ( a << n ) | (a >> (8-n));
}