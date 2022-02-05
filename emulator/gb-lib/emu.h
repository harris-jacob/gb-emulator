#ifndef EMU_H_
#define EMU_H_

//#include <SDL2/SDL.h>
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
    /* Is the emu in debug */
    bool debug;
    /*  */
    /* Screen context */
    // SDL_Window* window;
    // SDL_Renderer* renderer;
} emu_t;

/* create emulator instance, pass options here */
emu_t* emu_create(bool debug);

/** Single step of emulator - CPU and PPu */
void emu_step(emu_t* emu);

/** Load a rom */
void emu_load_rom(emu_t* emu, const char* filepath); 

#endif