#include "rom.h"
#include "stdio.h"
#include "stdlib.h"

// Eventually we want to do something with this
const char* rom_type_map[28] = {
    "ROM ONLY",
	"MBC1",
	"MBC1+RAM",
	"MBC1+RAM+BATTERY",
	"MBC2",
	"MBC2+BATTERY",
	"ROM+RAM",
	"ROM+RAM+BATTERY",
	"MMM01",
	"MMM01+RAM",
	"MMM01+RAM+BATTERY",
	"MBC3+TIMER+BATTERY",
	"MBC3+TIMER+RAM+BATTERY",
	"MBC3",
	"MBC3+RAM",
	"MBC3+RAM+BATTERY",
	"MBC4",
	"MBC4+RAM",
	"MBC4+RAM+BATTERY",
	"MBC5",
	"MBC5+RAM",
	"MBC5+RAM+BATTERY",
	"MBC5+RUMBLE",
	"MBC5+RUMBLE+RAM",
	"MBC5+RUMBLE+RAM+BATTERY",
	"POCKET CAMERA",
	"BANDAI TAMA5",
	"HuC3",
	"HuC1+RAM+BATTERY",
};

rom_t* rom_create() {
	rom_t* rom = (rom_t*)malloc(sizeof(rom_t));
	return rom;
}

void rom_destroy(rom_t** rom) {
	free(*rom);
	*rom = NULL;
}

// At the moment we're just assuming we load no more than 32Kbs (all the addressable memory we have)
// For future reference we will eventually implement various MCBs see: http://gbdev.gg8.se/wiki/articles/Memory_Bank_Controllers
rom_t* load_rom(const char* filepath) {
	FILE *fp;
	rom_t* rom = rom_create();

	fp = fopen(filepath, "r");
	
	if(fp == NULL) {
		perror("Error opening rom\n");
		exit(EXIT_FAILURE);


	// read the first 32kbs of the rom into memory
	size_t write_size = fread(rom->memory, 1, 32000, fp);
	printf("memory read: %u kBs");

	// ROM name is here in memory and is 16bytes long 
	for(int i=0; i<=16; i++){ 
		rom->name[i] = (char)rom->memory->addr[0x0100 + i];
		printf("%s");
	}
	
	return rom;
} 