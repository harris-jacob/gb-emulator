#include "unity.h"
#include "mmu.h"
#include "unity_fixture.h"

mmu_t* memory;

// Declare group
TEST_GROUP(mmu);

// Runs before each test
TEST_SETUP(mmu) {
    memory = mmu_create();
}

// Runs after each test
TEST_TEAR_DOWN(mmu) {
    if(memory != NULL) {
        mmu_destroy(&memory);
    }
}

/* The MMU create func should create an MMU instance */
TEST(mmu, mmu_create_ShouldCreateMMU) {
    TEST_ASSERT_NOT_NULL(memory);
}

/* The MMU destroy func should destroy an MMU instance */
TEST(mmu, mmu_destroy_ShouldDestroyMMU) {
    mmu_destroy(&memory);
    TEST_ASSERT_NULL(memory);
}

/* MMU write addr 8, should write ints to the allocated address */
TEST(mmu, mmu_write8_ShouldWriteToAddr) {
    mmu_write_addr8(memory, 0x1000, 12);
    TEST_ASSERT_EQUAL_UINT8(12, memory->addr[0x1000]);
}

/** MMU write addr 8 should write to non bios memory, if bios is not loaded & value is < 256 */
TEST(mmu, mmu_write16_ShouldWriteToAddr){
    mmu_write_addr16(memory, 0x1089, 1000);
     uint16_t val = *(uint16_t*)(memory->addr+0x1089);
    TEST_ASSERT_EQUAL_UINT16(1000, val);

}

/** MMU read addr 8 should read 8bit ints from memory */
TEST(mmu, mmu_read8_ShouldReadAddr) {
    memory->bios[10] = 100;
    memory->addr[10] = 250;
    memory->addr[0x1000] = 200;

    // Should read from bios if not complete
    *memory->finished_bios = false;
    uint8_t val = mmu_read_addr8(memory, 10);
    TEST_ASSERT_EQUAL_UINT8(100, val);
    // Should read from addr if val greater than 256
    val = mmu_read_addr8(memory, 0x1000);
    TEST_ASSERT_EQUAL_UINT8(200, val);
    // Should rad from addr if Bios has run
    *memory->finished_bios = true;
    val = mmu_read_addr8(memory, 10);
    TEST_ASSERT_EQUAL_UINT8(250, val);
}

/* MMU READ 16BIT should read from 2 8bit registers */
TEST(mmu, mmu_read16_ShouldReadAddr) {
    *((uint16_t*)(memory->bios+10)) = 1000;
    *((uint16_t*)(memory->addr+0x1000)) = 2000;
    *((uint16_t*)(memory->addr+10)) = 3000;
    // Should read from bios if not complete
    *memory->finished_bios = false;
    uint16_t val = mmu_read_addr16(memory, 10);
    TEST_ASSERT_EQUAL_UINT16(1000, val);
    // Should read from add if val is greater than 256
    val = mmu_read_addr16(memory, 0x1000);
    TEST_ASSERT_EQUAL_UINT16(2000, val);
    // Should rad from addr if Bios has run
    *memory->finished_bios = true;
    val = mmu_read_addr16(memory, 10);
    TEST_ASSERT_EQUAL_UINT16(3000, val);
}

/* The bios should load */
TEST(mmu, mmu_load_bios_ShouldLoadBios) {
    mmu_load_bios(memory);
    
    // Bios finished should be set to false
    TEST_ASSERT_FALSE(*memory->finished_bios);
    // Should have some bios values
    TEST_ASSERT_EQUAL_UINT8(0x31, mmu_read_addr8(memory, 0));
    TEST_ASSERT_EQUAL_UINT8(0xF0, mmu_read_addr8(memory, 100));
    TEST_ASSERT_EQUAL_UINT8(0xBB, mmu_read_addr8(memory, 200));
}

TEST_GROUP_RUNNER(mmu) {
    RUN_TEST_CASE(mmu, mmu_create_ShouldCreateMMU);
    RUN_TEST_CASE(mmu, mmu_destroy_ShouldDestroyMMU);
    RUN_TEST_CASE(mmu, mmu_write8_ShouldWriteToAddr);
    RUN_TEST_CASE(mmu, mmu_write16_ShouldWriteToAddr);
    RUN_TEST_CASE(mmu, mmu_read8_ShouldReadAddr);
    RUN_TEST_CASE(mmu, mmu_read16_ShouldReadAddr);
    RUN_TEST_CASE(mmu, mmu_load_bios_ShouldLoadBios);
}