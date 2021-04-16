#ifndef ROM_H_
#define ROM_H_
#include "mmu.h"
#include "stdio.h"
#include "stdlib.h"


/* ROM and all the necessary bits */
typedef struct rom_t_ {
    /* the rom needs access to memory */
    mmu_t* memory;
    /* Name of rom */
    char name[16];
    /* Catridge type of rom */
    uint8_t catridge_type;
    /* size of the rom */
    uint8_t rom_size;
    /* size of external ram in the cartridge */
    uint8_t ram_size;

} rom_t;


/* Create a rom instance */
rom_t* rom_create(mmu_t* mmu);
/* Destroy a rom instance */
void rom_destroy(rom_t** rom);

/* Map of rom types */
const char* rom_type_map[29];

/* Load a rom from disk */
rom_t* load_rom(const char* filepath, mmu_t* mmu);

/* used internally by load ROM to load the header data */
void load_rom_header(FILE* f, rom_t* rom);

#endif