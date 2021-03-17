#include "unity.h"
#include "mmu.h"

mmu_t* memory;


void setUp(void) {
    memory = mmu_create();
}

void tearDown(void) {
    if(memory != NULL) {
        mmu_destroy(&memory);
    }
}


void mmu_create_ShouldCreateMMU(void) {
    TEST_ASSERT_NOT_NULL(memory);
}

void mmu_destroy_ShouldDestroyMMU(void) {
    mmu_destroy(&memory);
    TEST_ASSERT_NULL(memory);
}


int main(void) {
    UNITY_BEGIN();
    RUN_TEST(mmu_create_ShouldCreateMMU);
    RUN_TEST(mmu_destroy_ShouldDestroyMMU);
    return UNITY_END();
}