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
bool should_halfcarry16(uint16_t prev, uint16_t curr);
/* should a carry be set 16bit */
bool should_carry16(uint16_t prev, uint16_t curr);
/* 
 * Adds two 8 bit numbers, sets register flags if necessary, 
 * returns the outcome
 */
uint8_t alu_add8(reg_t* reg, uint8_t a, uint8_t b);
/* 
 * Adds two 16 bit numbers, sets register flags if necessary, 
 * returns the outcome
 */
uint8_t alu_subtract8(reg_t* reg, uint8_t a, uint8_t b);
/* 
 * Adds two 8 bit numbers, sets register flags if necessary, 
 * returns the outcome
 */
uint16_t alu_add16(reg_t* reg, uint16_t a, uint16_t b);
/* 
 * Adds two 16 bit numbers, sets register flags if necessary, 
 * returns the outcome
 */
uint16_t alu_subtract16(reg_t* reg, uint16_t a, uint16_t b);

#endif