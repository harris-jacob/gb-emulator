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

/* The MMU create func should create an MMU instance */
void mmu_create_ShouldCreateMMU(void) {
    TEST_ASSERT_NOT_NULL(memory);
}

/* The MMU destroy func should destroy an MMU instance */
void mmu_destroy_ShouldDestroyMMU(void) {
    mmu_destroy(&memory);
    TEST_ASSERT_NULL(memory);
}

/* MMU write addr 8, should write ints to the allocated address */
void mmu_write8_ShouldWriteToAddr(void) {
    mmu_write_addr8(memory, 0x1000, 12);
    TEST_ASSERT_EQUAL_UINT8(12, memory->addr[0x1000]);
}

/** MMU write addr 8 should write to non bios memory, if bios is not loaded & value is < 256 */
void mmu_write16_ShouldWriteToAddr(void) {
    mmu_write_addr16(memory, 0x1089, 1000);
     uint16_t val = *(uint16_t*)(memory->addr+0x1089);
    TEST_ASSERT_EQUAL_UINT16(1000, val);

}

/** MMU read addr 8 should read 8bit ints from memory */
void mmu_read8_ShouldReadAddr(void) {
    memory->bios[10] = 100;
    memory->addr[0x1000] = 200;

    // Should read from bios if not complete
    uint8_t val = mmu_read_addr8(memory, 10);
    TEST_ASSERT_EQUAL_UINT8(100, val);
    // Should read from addr if val greater than 256
    val = mmu_read_addr8(memory, 0x1000);
    TEST_ASSERT_EQUAL_UINT8(200, val);
    // Should rad from addr if Bios has run
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(mmu_create_ShouldCreateMMU);
    RUN_TEST(mmu_destroy_ShouldDestroyMMU);
    RUN_TEST(mmu_write8_ShouldWriteToAddr);
    RUN_TEST(mmu_write16_ShouldWriteToAddr);
    RUN_TEST(mmu_read8_ShouldReadAddr);
    return UNITY_END();
}