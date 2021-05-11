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