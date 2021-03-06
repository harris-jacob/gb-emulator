#include "register.h"
#include "bit_utils.h"


reg_t* reg_create() {
	reg_t* registers = (reg_t*)malloc(sizeof(reg_t));

    return registers;
}

void reg_destroy(reg_t** registers) {
	free(*registers);
	*registers = NULL;
}

void set_carry(reg_t* reg) {
	SET_BIT(reg->f, 4);
}

void set_halfcarry(reg_t* reg) {
	SET_BIT(reg->f, 5);
}

void set_zero(reg_t* reg) {
	SET_BIT(reg->f, 7);
}

void set_subtract(reg_t* reg) {
	SET_BIT(reg->f, 6);
}


void reset_carry(reg_t* reg) {
	CLEAR_BIT(reg->f, 4);
}

void reset_halfcarry(reg_t* reg) {
	CLEAR_BIT(reg->f, 5);
}

void reset_zero(reg_t* reg) {
	CLEAR_BIT(reg->f, 7);
}

void reset_subtract(reg_t* reg) {
	CLEAR_BIT(reg->f, 6);
}

uint8_t get_carry(reg_t* reg) {
	return GET_BIT(reg->f, 4);
}

uint8_t get_halfcarry(reg_t* reg) {
	return GET_BIT(reg->f, 5);
}

uint8_t get_zero(reg_t* reg) {
	return GET_BIT(reg->f, 7);
}

uint8_t get_subtract(reg_t* reg) {
	return GET_BIT(reg->f, 6);
}

bool should_add_halfcarry8(uint8_t a, uint8_t b) {
	return (((a & 0xf) + (b & 0xf)) & 0x10);
}

bool should_add_halfcarry16(uint16_t a, uint16_t b) {
	return  (((a & 0xff) + (b & 0xff)) & 0x100);
}

bool should_add_carry8(uint8_t a, uint8_t b) {
	uint16_t out = a + b;
	return out > 0xff;
}

bool should_add_carry16(uint16_t a, uint16_t b) {
	uint32_t out = a + b;
	return out > 0xffff;
}

bool should_sub_halfcarry8(uint8_t a, uint8_t b) {
	return (((a & 0xf) - (b & 0xf)) & 0x10) == 0x10;
}

bool should_sub_carry8(uint8_t a, uint8_t b) {
		return b > a;
}

void alu_add8(reg_t* reg, uint8_t a) {
	reset_subtract(reg);

	if(should_add_carry8(reg->a, a)) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(should_add_halfcarry8(reg->a, a)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	reg->a= reg->a + a;
	
	if (reg->a == 0)
	{
		set_zero(reg);
	} else {
		reset_zero(reg);
	} 
}

void alu_adc8(reg_t* reg, uint8_t operand) {
	reset_subtract(reg);

	int val = operand + get_carry(reg);
	
	if(should_add_halfcarry8(reg->a, val)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	if(should_add_carry8(reg->a, val)) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	reg->a += (uint8_t)(val & 0xff);

	if(reg->a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}
}

void alu_sbc8(reg_t* reg, uint8_t operand) {
	set_subtract(reg);

	int val = operand + get_carry(reg);


	if(should_sub_halfcarry8(reg->a, val)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	if(should_sub_carry8(reg->a, val)) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}


	reg->a -= val;

	if(reg->a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}
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

void alu_subtract8(reg_t* reg, uint8_t a) {
	set_subtract(reg);

	if(should_sub_carry8(reg->a, a)) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(should_sub_halfcarry8(reg->a, a)) {
		set_halfcarry(reg);
	} else {
		reset_halfcarry(reg);
	}

	reg->a = reg->a - a;
	if (reg->a == 0)
	{
		set_zero(reg);
	} else {
		reset_zero(reg);
	} 
}


uint8_t alu_inc8(reg_t* reg, uint8_t a) {
	reset_subtract(reg);

	if(should_add_halfcarry8(a, 1)){
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

	if(a & 0xf) {
		reset_halfcarry(reg);
	} else {
		set_halfcarry(reg);
	}

	a--;

	if(a == 0 ) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	return a; 
}

uint8_t rrc(reg_t* reg, uint8_t a) {
	if(a & 1 ) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	a = ( a >> 1 ) | (a << 7);

	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	reset_halfcarry(reg);
	reset_subtract(reg);	

	return a;
}

uint8_t rlc(reg_t* reg, uint8_t a) {

	if(a & 0x80) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	a = ( a << 1 ) | (a >> 7);
	
	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	reset_halfcarry(reg);
	reset_subtract(reg);
	
	return a;
}


uint8_t rr(reg_t* reg, uint8_t a) {
	// new carry
	uint8_t carry = a & 1;

	// shift
	a = (a>> 1) | (get_carry(reg) << 7);
	

	if(carry) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}
	
	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	// reset
	reset_subtract(reg);
	reset_halfcarry(reg);

	return a;
}


uint8_t rl(reg_t* reg, uint8_t a) {
	// new carry
	uint8_t carry = (a & 128) >> 7;

	// shift
	a = a<<1 | get_carry(reg);
	

	if(carry) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}
	
	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	// reset
	reset_subtract(reg);
	reset_halfcarry(reg);

	return a;
}


uint8_t swap(reg_t* reg, uint8_t a) {
	a = ((a & 0x0f) << 4 | (a & 0xf0) >> 4);

	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	// reset flags
	reset_carry(reg);
	reset_halfcarry(reg);
	reset_subtract(reg);

	return a;
}

uint8_t sla(reg_t* reg, uint8_t a) {
	uint8_t carry = a >> 7;
	a <<= 1;

	if(carry) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	// reset
	reset_halfcarry(reg);
	reset_subtract(reg);

	return a;
}

uint8_t sra(reg_t* reg, uint8_t a) {
	uint8_t carry = a & 1;

	a = ((a & 128) | (a >> 1));

	if(carry) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	// reset
	reset_subtract(reg); 
	reset_halfcarry(reg);

	return a;
}

uint8_t set(uint8_t a, uint8_t n) {
	return a |= 1 <<n;
}

uint8_t reset(uint8_t a, uint8_t n) {
	return a &= ~(1 << n);
}

void bit(reg_t* reg, uint8_t a, uint8_t n) {
	if((a>>n)&1) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	// reset
	set_halfcarry(reg);
	reset_subtract(reg);
}

uint8_t srl(reg_t* reg, uint8_t a) {
	if(a & 1) {
		set_carry(reg);
	} else {
		reset_carry(reg);
	}

	a = a >> 1;

	if(a == 0) {
		set_zero(reg);
	} else {
		reset_zero(reg);
	}

	reset_halfcarry(reg);
	reset_subtract(reg);

	return a;
}

void cp(reg_t* reg, uint8_t val) {
	set_subtract(reg);
    if(val > reg->a) {
        set_carry(reg);
    } else {
        reset_carry(reg);
    }

    if((reg->a & 0x0f) < (val & 0x0f)) {
        set_halfcarry(reg);
    } else {
        reset_halfcarry(reg);
    }

    if(reg->a == val) {
        set_zero(reg);
    } else {
        reset_zero(reg);
    }
}