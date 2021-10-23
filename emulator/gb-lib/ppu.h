#ifndef PPU_H_
#define PPU_H_
#include "mmu.h"
#include "stdbool.h"

/* are the LCD and PPU enabled */
uint8_t is_lcd_enabled(mmu_t *mmu);

/*
 *  Get current mode of the ppu:
 *        0: In HBlank
 *        1: In VBlank
 *        2: Searching OAM
 *        3: Transferring Data to lcd Controller
 */
uint8_t get_gpu_mode(mmu_t *mmu);

/*
 * Bit 6 - LYC=LY Interrupt             (1=Enable) (Read/Write)
 * Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
 * Bit 4 - Mode 1 VBlank Interrupt      (1=Enable) (Read/Write)
 * Bit 3 - Mode 0 HBlank Interrupt      (1=Enable) (Read/Write)
 * Bit 2 - LYC=LY Flag      (0=Different, 1=Equal) (Read Only)
 */
void set_mmu_status(mmu_t *mmu);

/*
 * fetch tileset from memory, process it
 * and return it in a way the graphics engine can process
 */
uint8_t *fetch_tileset(mmu_t *mmu);

void ppu_clock_step(mmu_t *mmu, uint32_t instruction_time);

#endif