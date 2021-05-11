#include "rom.h"

// Eventually we want to do something with this
const char* rom_type_map[29] = {
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

rom_t* rom_create(mmu_t* mmu) {
	rom_t* rom = (rom_t*)malloc(sizeof(rom_t));
	rom->memory = mmu;
	return rom;
}

void rom_destroy(rom_t** rom) {
	free(*rom);
	*rom = NULL;
}

// At the moment we're just assuming we load no more than 32Kbs (all the addressable memory we have)
// For future reference we will eventually implement various MCBs see: http://gbdev.gg8.se/wiki/articles/Memory_Bank_Controllers
rom_t* load_rom(const char* filepath, mmu_t* mmu) {
	FILE *fp;
	rom_t* rom = rom_create(mmu);

	// OPEN ROM
	fp = fopen(filepath, "rb");
	
	if(fp == NULL) {
		perror("Error opening rom\n");
		exit(EXIT_FAILURE);
	}

	// read header
	load_rom_header(fp, rom);

	size_t write_size = fread(rom->memory->addr, sizeof(uint8_t), 32000, fp);
	printf("Rom data - memory read: %lu Bs\n", write_size);
	
	return rom;
}


void load_rom_header(FILE* f, rom_t* rom) {

	char header[0x180]; 
	
	// read the 385 bytes that contines the header info
	size_t write_size = fread(header, 0x17f, 1, f);

	// ROM name is here in memory and is 16bytes long 
	for(int i=0; i<=16; i++) { 
		rom->name[i] = header[0x134 + i];
	}

	// set other header values
	rom->catridge_type = header[0x147];
	rom->rom_size = header[0x148];
	rom->ram_size = header[0x149];

	// Move the file pointer back
	fseek(f, 0, SEEK_SET);
}

