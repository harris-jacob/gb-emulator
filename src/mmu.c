#include "mmu.h"



mmu_t* mmu_create() {
	mmu_t* mmu = (mmu_t*)malloc(sizeof(mmu_t));
	return mmu;
}

void mmu_destroy(mmu_t	**mmu) {
	free(*mmu);
	*mmu = NULL;
}

uint8_t mmu_read_addr8(mmu_t* mmu, uint16_t addr) {
	return *(mmu->addr + addr);
}

void mmu_write_addr8(mmu_t* mmu, uint16_t addr, uint8_t data) {	
	mmu->addr[addr] = data;
}

uint16_t mmu_read_addr16(mmu_t* mmu, uint16_t addr) {
	return *((uint16_t*)(mmu->addr + addr));
}

void mmu_write_addr16(mmu_t* mmu, uint16_t addr, uint16_t data) {
	uint16_t* pos = ((uint16_t*)(mmu->addr + addr));
	*pos = data;
}