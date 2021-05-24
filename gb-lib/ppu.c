#include "ppu.h"
#include "bit_utils.h"

#define TILE_SIZE 16
#define TILE_NO 384
#define SCREEN_WIDTH 160
#define SCREEN_HEIGHT 144

// TODO move to ppu_t
uint32_t ppu_clock;

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

uint8_t is_lcd_enabled(mmu_t* mmu) {
    uint8_t val = mmu_read_addr8(mmu, 0xff40);

    return GET_BIT(val, 7);
}

uint8_t get_ppu_mode(mmu_t* mmu) {
    uint8_t val = mmu_read_addr8(mmu, 0xff41);

    return val & 0x3;

    return 0;
}

void set_ppu_mode(mmu_t* mmu, uint32_t mode) {
    uint8_t curr = mmu_read_addr8(mmu, 0xff41);
    uint8_t new;
    uint8_t int_val;
    switch (mode)
    {
    case 1: ;
        int_val = SET_BIT(curr, 0);
        new = CLEAR_BIT(int_val, 1);
        break;
    case 2: ;
        int_val = CLEAR_BIT(curr, 0);
        new = SET_BIT(int_val, 1)
        break;
    case 3: ;
        int_val = SET_BIT(curr, 0);
        new = SET_BIT(int_val, 1);
        break;
    default: ;
        int_val = CLEAR_BIT(curr, 0);
        new = CLEAR_BIT(int_val, 1);
    }

    mmu_write_addr8(mmu, 0xff41, new);
}


// attribution: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
void ppu_clock_step(mmu_t* mmu, uint32_t clock) {
    
    uint8_t mode = get_ppu_mode(mmu);
    
    static uint32_t lastTicks = 0;

    ppu_clock += clock - lastTicks;

    lastTicks = clock;

    switch (mode)
    {
    // hblank
    case 0:
        if(ppu_clock >= 204) {
            ppu_clock-=204;
            
            // update ly val
            mmu->addr[0xff44]++;

            // enter vblank
            if(mmu->addr[0xff44] == 143) {
                set_ppu_mode(mmu, 1);
                // render
            } else {
                set_ppu_mode(mmu, 2);
            }
        }
        break;
    // OAM read mode, scanline active
    case 2:
        if(ppu_clock >= 80) {
            ppu_clock-= 80;
            set_ppu_mode(mmu, 3);
        }
        break;

    // vram read mode
    case 3:
        if(ppu_clock >= 172) {
            // enter hblank
            set_ppu_mode(mmu, 0);
            ppu_clock-=172;

            // write scanline
        }
        break;

    // vblank
    case 1:
        if(ppu_clock >= 456) {
            ppu_clock-=456;
            mmu->addr[0xff44]++;

            if(mmu->addr[0xff44] > 153) {
                set_ppu_mode(mmu, 2);
                mmu->addr[0xff44] = 0; 
            }
        }
        break;
    
    default:
        break;
    }
}