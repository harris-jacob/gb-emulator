#include "unity.h"
#include "register.h"
#include "unity_fixture.h"


reg_t* reg;

// Register group
TEST_GROUP(bit);

// Runs before each test
TEST_SETUP(bit) {
    reg = reg_create();
}

// Runs after each test
TEST_TEAR_DOWN(bit) {
    if(reg != NULL) {
        reg_destroy(&reg);
    }
}

TEST(bit, bit_SwapShouldSwapNibbles) {

    uint8_t a = 0b10110000;
    uint8_t val1 = swap(reg, a); 
    uint8_t b = 0b01101010;
    uint8_t val2 = swap(reg, b); 
    
    TEST_ASSERT_EQUAL_UINT8(0b00001011, val1);
    TEST_ASSERT_EQUAL_UINT8(0b10100110, val2);
}


TEST(bit, bit_SwapShouldHandleFlags) {

    reg->a = 0;
    reg->f = 0b01110000;

    uint8_t val2 = swap(reg, reg->a); 
    
    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b10000000, reg->f);
}

TEST(bit, bit_SetShouldSetBits) {

    uint8_t a = 0b00000000;
    uint8_t val1 = set(a, 2); 
    uint8_t val2 = set(val1, 6); 
    uint8_t val3 = set(val2, 7);
    
    TEST_ASSERT_EQUAL_UINT8(0b00000100, val1);
    TEST_ASSERT_EQUAL_UINT8(0b01000100, val2);
    TEST_ASSERT_EQUAL_UINT8(0b11000100, val3);
}

TEST(bit, bit_ResetShouldResetBits) {

    uint8_t a = 0b00110001;
    uint8_t val1 = reset(a, 0); 
    uint8_t val2 = reset(val1, 4); 
    uint8_t val3 = reset(val2, 5);
    
    TEST_ASSERT_EQUAL_UINT8(0b00110000, val1);
    TEST_ASSERT_EQUAL_UINT8(0b00100000, val2);
    TEST_ASSERT_EQUAL_UINT8(0b00000000, val3);
}

TEST(bit, bit_BitShouldPopulateCarry) {

    reg->a = 0b01000100;
    reg->f = 0;
    bit(reg, reg->a, 2);

    TEST_ASSERT_EQUAL_UINT8(0b00110000, reg->f);
    TEST_ASSERT_EQUAL_UINT8(0b01000100, reg->a);
    // Reset
    bit(reg, reg->a, 4);

    TEST_ASSERT_EQUAL_UINT8(0b00100000, reg->f);

}

TEST(bit, bit_BitShouldSetFlags) {
    reg->a = 0;
    reg->f = 0b01110000;
    bit(reg, reg->a, 2);

    TEST_ASSERT_EQUAL_UINT8(0b10100000, reg->f);
}

TEST_GROUP_RUNNER(bit) {
    // SWAP
    RUN_TEST_CASE(bit, bit_SwapShouldSwapNibbles);
    RUN_TEST_CASE(bit, bit_SwapShouldHandleFlags);

    // SET
    RUN_TEST_CASE(bit, bit_SetShouldSetBits);
    
    // RESET 
    RUN_TEST_CASE(bit, bit_ResetShouldResetBits);

    // BIT
    RUN_TEST_CASE(bit, bit_BitShouldPopulateCarry);
    RUN_TEST_CASE(bit, bit_BitShouldSetFlags);
}