// taken from https://www.linkedin.com/pulse/creating-gameboy-emulator-part-1-bruno-croci/
#ifndef MMU_H_
#define MMU_H_
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

/*
 * 16 bit addressable memory space
 */
typedef struct _mmu_t
{
		// union allows us to access each segment of memory directly
		union
		{
			uint8_t addr[0x10000];

			struct
			{	
				// Two ROM slots
				uint8_t rom[2][0x4000];
				// Video ram
				uint8_t vram[0x2000];
				// Switchable RAM bank
				uint8_t sram[0x2000];
				// Internal RAM
				uint8_t iram[0x2000];
				// Unusable
				uint8_t wrams[0x1E00];
				// Sprite Attributes
				uint8_t sprites[0xA0];
				// Unusable slot
				uint8_t empty[0x60];
				// I/0
				uint8_t io[0x40];
				// Untouchable
				uint8_t ppu[0x40];
				// High RAM
				uint8_t hram[0x80];
				// Interupt enable
				uint8_t interrupt_enable;
			};
		};
} mmu_t;


/* create a 16bit set of accessible memory */ 
mmu_t* mmu_create();

/* initialize memory values as if boot rom had run */
void mmu_init(mmu_t* mmu);

/* destroy set of memory */
void mmu_destroy(mmu_t** mmu);

/* read 8 bit values of memory */
uint8_t mmu_read_addr8(mmu_t* mmu, uint16_t addr);

/* write 8 bit value of memory */
void mmu_write_addr8(mmu_t* mmu, uint16_t addr, uint8_t data);

/* read 16bit value */
uint16_t mmu_read_addr16(mmu_t* mmu, uint16_t addr);

/* write 16bit value */
void mmu_write_addr16(mmu_t* mmu, uint16_t addr, uint16_t data);

/* Dump the contents of memory into a file */
void mmu_mem_dump(mmu_t* mmu);

/* set vblank enable */
static inline void set_vblank_enable(mmu_t* mmu) {
	mmu->interrupt_enable |= 1;
}
/* set lcdstat reg */
static inline void set_lcdstat_enable(mmu_t* mmu) {
	mmu->interrupt_enable |= (1 << 1);
}
/* set timer enable */
static inline void set_timer_enable(mmu_t* mmu) {
	mmu->interrupt_enable |= (1 << 2);
}
/* set serial enable */
static inline void set_serial_enable(mmu_t* mmu) {
	mmu->interrupt_enable |= (1 << 3);
}
/* set joypad enable */
static inline void set_joypad_enable(mmu_t* mmu) {
	mmu->interrupt_enable |= (1 << 4);
}
/* reset vblank enable */
static inline void reset_vblank_enable(mmu_t* mmu) {
	mmu->interrupt_enable &= ~(1);
}
/* reset lcdstat enable */
static inline void reset_lcdstat_enable(mmu_t* mmu) {
	mmu->interrupt_enable&= ~(1 << 1);
}
/* reset timer enable */
static inline void reset_timer_enable(mmu_t* mmu) {
	mmu->interrupt_enable&= ~(1 << 2);
}
/* reset serial enable */
static inline void reset_serial_enable(mmu_t* mmu) {
	mmu->interrupt_enable&= ~(1 << 3);
}
/* reset joypad enable */
static inline void reset_joypad_enable(mmu_t* mmu) {
	mmu->interrupt_enable&= ~(1 << 4);
}
/* set vblank */
static inline void set_vblank(mmu_t* mmu) {
	mmu->addr[0xFF0F]|= 1;
}
/* set lcdstat */
static inline void set_lcdstat(mmu_t* mmu) {
	mmu->addr[0xFF0F]|= (1 << 1);
}
/* set timer */
static inline void set_timer(mmu_t* mmu) {
	mmu->addr[0xFF0F]|= (1 << 2);
}
/* set serial */
static inline void set_serial(mmu_t* mmu) {
	mmu->addr[0xFF0F]|= (1 << 3);
}
/* set joypad */
static inline void set_joypad(mmu_t* mmu) {
	mmu->addr[0xFF0F]|= (1 << 4);
}
/* reset vblank */
static inline void reset_vblank(mmu_t* mmu) {
	mmu->addr[0xFF0F] &= ~(1);
}
/* reset lcdstat */
static inline void reset_lcdstat(mmu_t* mmu) {
	mmu->addr[0xFF0F] &= ~(1 << 1);
}
/* reset timer */
static inline void reset_timer(mmu_t* mmu) {
	mmu->addr[0xFF0F] &= ~(1 << 2);
}
/* reset serial */
static inline void reset_serial(mmu_t* mmu) {
	mmu->addr[0xFF0F]&= ~(1 << 3);
}
/* reset joypad */
static inline void reset_joypad(mmu_t* mmu) {
	mmu->addr[0xFF0F]&= ~(2 << 4);
}

#endif