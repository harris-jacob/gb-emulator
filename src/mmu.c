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

void mmu_disable_all_interrupts(mmu_t* mmu) {
	mmu->interupts = 0;
}

void mmu_enable_all_interrupts(mmu_t* mmu) {
	mmu->interupts = 0x100;
}

void set_vblank(mmu_t* mmu) {
	mmu->interupts |= 1;
}

void set_lcdstat(mmu_t* mmu) {
	mmu->interupts |= (1 << 1);
}

void set_timer(mmu_t* mmu {
	mmu->interupts |= (1 << 2);
}

void set_serial(mmu_t* mmu) {
	mmu->interupts |= (1 << 3);
}

void set_joypad(mmu_t* mmu) {
	mmu->interupts |= (1 << 4);
}

void reset_vblank(mmu_t* mmu) {
	mmu->interupts &= ~(1);
}

void reset_lcdstat(mmu_t* mmu) {
	mmu->interupts &= ~(1 << 1);
}

void reset_timer(mmu_t* mmu) {
	mmu->interupts &= ~(1 << 2);
}

void reset_serial(mmu_t* mmu) {
	mmu->interupts &= ~(1 << 3);
}

void reset_joypad(mmu_t* mmu) {
	mmu->interupts &= ~(1 << 4);
}