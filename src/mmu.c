#include "mmu.h"



mmu_t* mmu_create() {
	mmu_t* mmu = (mmu_t*)malloc(sizeof(mmu_t));
	mmu->finished_bios = mmu->addr + 0xFF50;
	return mmu;
}

void mmu_destroy(mmu_t	**mmu) {
	free(*mmu);
	*mmu = NULL;
}

uint8_t mmu_read_addr8(mmu_t* mmu, uint16_t addr) {
	if(!(*mmu->finished_bios) && addr >= 0x00 && addr <= 0xFF)
		return *(mmu->bios + addr);


	return *(mmu->addr + addr);
}

void mmu_write_addr8(mmu_t* mmu, uint16_t addr, uint8_t data) {	
	mmu->addr[addr] = data;
}

uint16_t mmu_read_addr16(mmu_t* mmu, uint16_t addr) {
	if (!(*mmu->finished_bios) && addr >= 0x00 && addr <= 0xFF)
		return *((uint16_t*)(mmu->bios + addr));
	
	return *((uint16_t*)(mmu->addr + addr));
}

void mmu_write_addr16(mmu_t* mmu, uint16_t addr, uint16_t data) {
	uint16_t* pos = ((uint16_t*)(mmu->addr + addr));
	*pos = data;
}

void mmu_load_bios(mmu_t* mmu) {
	memcpy((void*)mmu->bios, (const void*)BIOS, sizeof(BIOS));
	(*mmu->finished_bios) = false;
}