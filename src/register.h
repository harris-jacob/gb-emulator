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


/* Should a half carry be set during addition 8bit  */
bool should_add_halfcarry8(uint8_t prev, uint8_t curr);
/* should a carry be set during addition 8bit */
bool should_add_carry8(uint8_t prev, uint8_t curr);
/* Should a half carry be set during addition 16bit  */
bool should_add_halfcarry16(uint16_t prev, uint16_t curr);
/* should a carry be set during addition 16bit */
bool should_add_carry16(uint16_t prev, uint16_t curr);
/* Should a half carry be set during subtract 8bit  */
bool should_sub_halfcarry8(uint8_t prev, uint8_t curr);
/* should a carry be set during subtract 8bit */
bool should_sub_carry8(uint8_t prev, uint8_t curr);
/* Should a half carry be set during subtract 16bit  */
bool should_sub_halfcarry16(uint16_t prev, uint16_t curr);
/* should a carry be set during subtract 16bit */
bool should_sub_carry16(uint16_t prev, uint16_t curr);
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
* Increment a uint8, set necessary flags
* returns outcome
*/
uint8_t alu_inc8(reg_t* reg, uint8_t a);

/*
* Decrement a uint8, set necesarry flags
* returns outcome
*/
uint8_t alu_dec8(reg_t* reg, uint8_t a);

/*
* 8 bit addition which also adds the carry flag to the output
* handles flags returns outcome.
*/
uint8_t alu_adc8(reg_t* reg, uint8_t a, uint8_t b);

/*
* 8 bit subtraction which also adds the carry flag to the output
* handles flags returns outcome.
*/
uint8_t alu_sbc8(reg_t* reg, uint8_t a, uint8_t b);

/*
* Rotate right - rotate through carry
*/
uint8_t rr(reg_t* reg, uint8_t n);

/*
* Rotate left - rotate through carry
*/
uint8_t rl(reg_t* reg, uint8_t n);

/* 
* Rotate left circular
*/
uint8_t rlc(reg_t* reg, uint8_t a);

/*
* Rotate right circular
*/
uint8_t rrc(reg_t* reg, uint8_t a);

/* Swap upper and lower nibbles of a */
uint8_t swap(reg_t* reg, uint8_t a);

/* Left arithmetic shift */
uint8_t sla(reg_t* reg, uint8_t a);

/* Right arithmetic shift */
uint8_t sla(reg_t* reg, uint8_t a);

/* Set nth bit of a */
uint8_t set(uint8_t a, uint8_t n);

/* Reset nth bit of a */
uint8_t reset(uint8_t a, uint8_t n);

#endif