#ifndef PPU_H_
#define PPU_H_
#include "mmu.h"
#include "stdbool.h"

/* are the LCD and PPU enabled */
uint8_t is_ppu_enabled(mmu_t *mmu);

/* which background map is sed for rendering if 0 then 9800-9BFF is used, if 1
 * 9C00-9FFF is used */
uint8_t window_tilemap_area(mmu_t *mmu);

/* is the window enabled or not? */
uint8_t is_window_enabled(mmu_t *mmu);

/* which addressing mode the bg and window use to pick tiles - if 0 then
 * 8800-97FF, if 1 then 9C00-9FFF */
uint8_t bg_window_tile_area(mmu_t *mmu);

/* controls which bits tilemap uses- if 0 then 9800 is used, if 1 then 9c00 is
 * used */
uint8_t bg_tile_map_area(mmu_t *mmu);

/* if 0 8x8 is used, if 1 8x16 is used */
uint8_t get_obj_size(mmu_t *mmu);

/* toggles whether sprites are shown. 0=off, 1=on */
uint8_t is_obj_enabled(mmu_t *mmu);

/* different meanings depending on mode */
uint8_t bg_window_priority(mmu_t *mmu);

/*
 * Bit 6 - LYC=LY Interrupt             (1=Enable) (Read/Write)
 * Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
 * Bit 4 - Mode 1 VBlank Interrupt      (1=Enable) (Read/Write)
 * Bit 3 - Mode 0 HBlank Interrupt      (1=Enable) (Read/Write)
 * Bit 2 - LYC=LY Flag      (0=Different, 1=Equal) (Read Only)

 *        0: In HBlank
 *        1: In VBlank
 *        2: Searching OAM
 *        3: Transferring Data to lcd Controller
 */
void set_mmu_status(mmu_t *mmu);

/*
 * fetch tileset from memory, process it
 * and return it in a way the graphics engine can process
 */
uint8_t *fetch_tileset(mmu_t *mmu);

#endif