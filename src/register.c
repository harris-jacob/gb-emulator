#include "register.h"


reg_t* reg_create() {
	reg_t* registers = (reg_t*)malloc(sizeof(reg_t));
    return registers;
}

void reg_destroy(reg_t** registers) {
	free(*registers);
	*registers = NULL;
}