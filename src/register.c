#include "register.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>


reg* mmu_create() {
	reg* registers = (reg*)malloc(sizeof(reg));
    return registers;
}

void mmu_destroy(reg* registers) {
	free(registers);
	registers = NULL;
}