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

TEST(bit, bit_rr_ShouldRotateBitsThroughCarry) {
    reg->f = 0b10010000;
    reg->a = 0b11001010;
    reg->a = rr(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b11100101, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0, reg->f);
}

TEST(bit, bit_rr_ShouldSetZeroAndCarry) {
    reg->f = 0b00000000;
    reg->a = 0b00000001;
    reg->a = rr(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(144, reg->f);
}

TEST_GROUP_RUNNER(bit) {

    // RR
    RUN_TEST_CASE(bit, bit_rr_ShouldRotateBitsThroughCarry);
    RUN_TEST_CASE(bit, bit_rr_ShouldSetZeroAndCarry);

    // LR

}
