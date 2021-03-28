#ifndef REGISTER_H_
#define REGISTER_H_
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>


typedef struct _reg_t
{
	struct {
		union {
			struct {
				uint8_t f;
				uint8_t a;
			};
			uint16_t af;
		};
	};
	
	struct {
		union {
			struct {
				uint8_t c;
				uint8_t b;
			};
			uint16_t bc;
		};
	};
	
	struct {
		union {
			struct {
				uint8_t e;
				uint8_t d;
			};
			uint16_t de;
		};
	};
	
	struct {
		union {
			struct {
				uint8_t l;
				uint8_t h;
			};
			uint16_t hl;
		};
	};
	
	uint16_t sp;
	uint16_t pc;
} reg_t;


/* Create a set of CPU registers */
reg_t* reg_create();

/* Destroy register */
void reg_destroy(reg_t** registers);

/* 
* Perform increment on uint8 in register
* handles setting flags
*/ 
void reg_inc8(reg_t* reg, uint8_t* inc_Reg);

/* 
* Perform decrement on uint8 in register
* handles flag setting
*/
void reg_dec8(reg_t* reg, uint8_t* inc_reg);

/*
* Perform 16 bit increment on a uint16 in register.
* handles flag setting.
*/
void reg_inc16(reg_t* reg, uint8_t* inc_reg);

/*
* Perform 16 bit decrement on a given in register.
* handles flag setting.
*/
void reg_dec16(reg_t* reg, uint8_t* inc_reg);


// FLAG SETTERS
void set_carry(reg_t* reg);
void set_halfcarry(reg_t* reg);
void set_zero(reg_t* reg);
void set_subtract(reg_t* reg);

// FLAG RESETERS
void reset_carry(reg_t* reg);
void reset_halfcarry(reg_t* reg);
void reset_zero(reg_t* reg);
void reset_subtract(reg_t* reg);

// FLAG GETTERS
uint8_t get_carry(reg_t* reg);
uint8_t get_halfcarry(reg_t* reg);
uint8_t get_zero(reg_t* reg);
uint8_t get_subtract(reg_t* reg);


/* Should a half carry be set 8bit  */
bool should_halfcarry8(uint8_t prev, uint8_t curr);
/* should a carry be set 8bit */
bool should_carry8(uint8_t prev, uint8_t curr);
/* Should a half carry be set 16bit  */
bool should_halfcarry16(uint8_t prev, uint8_t curr);
/* should a carry be set 16bit */
bool should_carry16(uint8_t prev, uint8_t curr);


#endif