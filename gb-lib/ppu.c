#include "ppu.h"
#include "bit_utils.h"

#define TILE_SIZE 16
#define TILE_NO 384
#define SCREEN_WIDTH 160
#define SCREEN_HEIGHT 144

uint8_t* fetch_tileset(mmu_t* mmu) {
    // pointer to the start of tileset 1
    uint8_t* tile_set = (mmu->addr + 0x8000);
    uint8_t* tiles[TILE_NO];


    // the output pixel data
    uint8_t* screen_data = (uint8_t*)malloc(SCREEN_HEIGHT * SCREEN_WIDTH * sizeof(uint8_t));

    for(int i=0; i<=384; i++) {
        tiles[i] = (tile_set + i*16);
    }

    // construct the out image
    int screen_ptr = 0;
    for(int i=0; i<= TILE_NO; i++) {
        for(int j=0; j< TILE_SIZE/2; j++) {
            // get tile row
            uint8_t first_bits = tiles[i][j];
            uint8_t second_bits = tiles[i][j+1];

            // evaluate bits
            for(int k=0; k< 8; k++) {
            uint8_t bit_1 = GET_BIT(first_bits, k);
            uint8_t bit_2 = GET_BIT(second_bits, k);
            uint8_t val = 0;
            if (bit_1 && bit_2) {
                val = 3;
            } else if(bit_1 && !bit_2) {
                val = 1;
            } else if(!bit_1 && bit_2) {
                val = 2;
            }

            screen_data[screen_ptr++] =  val;
            
            // for now break if we go out of bounds
            if(screen_ptr >= SCREEN_HEIGHT * SCREEN_WIDTH) {
                return screen_data;
            }
            }
        }
    } 

    return screen_data;

}
