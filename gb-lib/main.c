#include <SDL2/SDL.h>
#include <stdio.h>
#include <SDL.h>
#include "emu.h"
#include "ppu.h"
#include "bit_utils.h"

#define SCREEN_WIDTH 160*4
#define SCREEN_HEIGHT 144*4


void render(SDL_Renderer* renderer, uint8_t* tileset) {
    for(int i=0; i < SCREEN_HEIGHT; i++) {
        for(int j=0; j < SCREEN_WIDTH; j++) {
        int val = 255;
        switch(tileset[i]) {
            case 1:
                val = 0xcc;
                break;
            case 2: 
                val = 0x77;
                break;
            case 3:
                val = 0;
                break;
        }

        SDL_SetRenderDrawColor(renderer, val, val, val, 255);
        SDL_RenderDrawPoint(renderer, j, i);
        }
    }
    SDL_RenderPresent(renderer);
    SDL_Delay(1000 / 60);
}
 
int main(int argc, char ** argv) {

    uint8_t* tileset;
    emu_t* emu = emu_create();
    load_rom("./roms/tetris.gb", emu->mmu);

    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window* window = SDL_CreateWindow("gb emu",
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH, SCREEN_HEIGHT, 0);

    SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    SDL_RenderSetScale(renderer, 4, 4);

    // render loop
    for(int i=0; i<= 10000000; i++) {
        uint32_t clock_delta = cpu_step(emu->cpu);
        ppu_clock_step(emu->cpu->mmu, clock_delta);
    }


    mmu_mem_dump(emu->mmu);
    for(; ;) {
        tileset = fetch_tileset(emu->mmu);
        render(renderer, tileset);
    }

    return 0;
}