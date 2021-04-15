#ifndef ROM_H_
#define ROM_H_
#include "mmu.h"

/* ROM and all the necessary bits */
typedef struct rom_t_ {
    /* the rom needs access to memory */
    mmu_t* memory;
    /* Type of cartridge */
    char* type;
    /* Name of rom */
    char name[16];



} rom_t;


/* Create a rom instance */
rom_t* rom_create();
/* Destroy a rom instance */
void rom_destroy(rom_t** rom);

/* Map of rom types */
const char* rom_type_map[28];

/* Load a rom from disk */
rom_t* load_rom(const char* filepath);



#endif