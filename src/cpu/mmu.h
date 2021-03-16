// taken from https://www.linkedin.com/pulse/creating-gameboy-emulator-part-1-bruno-croci/
#ifndef MMU_H_
#define MMU_H_
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

/* 
 * CPU is 8bit, registers can be accessed in pairs. A, F, B, C, 
 * H can be accessed AF, BC and HL. PC and SP are 16 bit only.
 * 
 */
typedef struct _mmu_t
{
	// 256byte boot rom: hecks the cartridge header is correct, scrolls the Nintendo bootup graphics and plays the "po-ling" sound.
	uint8_t bios[0x100];
	struct
	{
		// union allows us to access each segment of memeory directly
		union
		{
			uint8_t addr[0x10000];

			struct
			{
				uint8_t rom[2][0x4000];
				uint8_t vram[0x2000];
				uint8_t eram[0x2000];
				uint8_t wram[0x2000];
				uint8_t wrams[0x1E00];
				uint8_t oam[0xA0];
				uint8_t empty[0x60];
				uint8_t io[0x40];
				uint8_t ppu[0x40];
				uint8_t zram[0x80];
				uint8_t intenable;
			};
		};
	};
	
	// pointer to the start address
	uint8_t* finished_bios;
} mmu_t;


/* create a 16bit set of accessible memory */ 
mmu_t* mmu_create();

/* destroy set of memory */
void mmu_destroy(mmu_t* mmu);

/* read 8 bit values of memory */
uint8_t mmu_read_addr8(mmu_t* mmu, uint16_t addr);

/* write 8 bit value of memory */
void mmu_write_addr8(mmu_t* mmu, uint16_t addr, uint8_t data);
/* read 16bit value */
uint16_t mmu_read_addr16(mmu_t* mmu, uint16_t addr);

/* write 16bit value */
void mmu_write_addr16(mmu_t* mmu, uint16_t addr, uint16_t data);

#endif