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

void mmu_init(mmu_t* mmu) {
	mmu->addr[0xff05] = 0;
	mmu->addr[0xff06] = 0;
	mmu->addr[0xff07] = 0;
	mmu->addr[0xff10] = 0x80;
	mmu->addr[0xff11] = 0xbf;
	mmu->addr[0xff12] = 0xF3;
	mmu->addr[0xff14] = 0xbf;
	mmu->addr[0xff16] = 0x3f;
	mmu->addr[0xff17] = 0;
	mmu->addr[0xff19] = 0xbf;
	mmu->addr[0xff1a] = 0x7f;
	mmu->addr[0xff1b] = 0xff;
	mmu->addr[0xff1c] = 0x9f;
	mmu->addr[0xff1e] = 0xbf;
	mmu->addr[0xff20] = 0xff;
	mmu->addr[0xff21] = 0;
	mmu->addr[0xff22] = 0;
	mmu->addr[0xff23] = 0xbf;
	mmu->addr[0xff24] = 0x77;
	mmu->addr[0xff25] = 0xF3;
	mmu->addr[0xff26] = 0xF1; // ??
	mmu->addr[0xff40] = 0x91;
	mmu->addr[0xff42] = 0;
	mmu->addr[0xff43] = 0;
	mmu->addr[0xff45] = 0;
	mmu->addr[0xff47] = 0xfc;
	mmu->addr[0xff48] = 0xff;
	mmu->addr[0xff49] = 0xff;
	mmu->addr[0xff4a] = 0;
	mmu->addr[0xff4b] = 0;
	mmu->addr[0xffff] = 0;

	
}

void mmu_mem_dump(mmu_t* mmu) {
	FILE *fptr;

	fptr = fopen("./mem_out.txt", "w");

	if(fptr == NULL) {
		// couldn't create the file
		printf("Error: Unable to create memory dump file.\n");
		return;
	}

	for(int i=0; i< sizeof(mmu_t);) {
		fprintf(fptr, "0x%x:  %x, %x, %x, %x, %x, %x, %x, %x \n", 
			i,
			mmu->addr[i],
			mmu->addr[i+1],
			mmu->addr[i+2],
			mmu->addr[i+3],
			mmu->addr[i+4],
			mmu->addr[i+5],
			mmu->addr[i+6],
			mmu->addr[i+7]
		);
		i+=7;
	}

	fclose(fptr);
	printf("Success: MMU dump file written\n");
}