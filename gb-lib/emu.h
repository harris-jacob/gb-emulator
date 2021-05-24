#ifndef EMU_H_
#define EMU_H_

#include <SDL2/SDL.h>
#include "cpu.h"
#include "mmu.h"
#include "rom.h"

typedef struct emu_t_ {
    /* CPU */
    cpu_t* cpu;
    /* ROM */
    rom_t* rom;
    /* MMU */
    mmu_t* mmu;
    /* Screen context */
    SDL_Window* window;
    SDL_Renderer* renderer;

} emu_t;

/* create emulator instance, pass options here */
emu_t* emu_create();

#endif