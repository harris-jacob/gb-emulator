#include "unity.h"
#include "register.h"
#include "unity_fixture.h"


reg_t* reg;

// Register group
TEST_GROUP(alu);

// Runs before each test
TEST_SETUP(alu) {
    reg = reg_create();
}

// Runs after each test
TEST_TEAR_DOWN(alu) {
    if(reg != NULL) {
        reg_destroy(&reg);
    }
}

TEST(alu, alu_add8_ShouldSetHalfCarry) {
    reg->f = 0b10000000;
    uint8_t val = alu_add8(reg, 11, 5);

    TEST_ASSERT_EQUAL_UINT8(16, val);
    TEST_ASSERT_EQUAL_UINT8(0b00100000, reg->f);
    
}

TEST(alu, alu_add8_ShouldSetCarry) {
    reg->f = 0b11000000;
    uint8_t val = alu_add8(reg, 0xff, 128);

    TEST_ASSERT_EQUAL_UINT8(127, val);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
    
}

TEST(alu, alu_add16_ShouldSetHalfCarry) {
    reg->f = 0b00000000;
    uint16_t val = alu_add16(reg, 0xff, 10);

    TEST_ASSERT_EQUAL_UINT16(265, val);
    TEST_ASSERT_EQUAL_UINT8(0b00100000, reg->f);
}

TEST(alu, alu_add16_ShouldSetCarry) {
    reg->f = 0b00000000;
    uint16_t val = alu_add16(reg, 0b01100000000000000, 0b001000000000000000);

    TEST_ASSERT_EQUAL_UINT16(16384, val);
    TEST_ASSERT_EQUAL_UINT16(16, reg->f);
}

TEST(alu, inc_ShouldIncReg) {
    reg->f = 0b01000000;

    uint8_t val = alu_inc8(reg, 9);

    TEST_ASSERT_EQUAL_UINT8(10, val);
    TEST_ASSERT_EQUAL_UINT8(0,reg->f);
}

TEST(alu, inc_ShouldSetHalfCarry) {
    reg->f = 0b00000000;

    uint8_t val = alu_inc8(reg, 0b00001111);

    TEST_ASSERT_EQUAL_UINT8(16, val);
    TEST_ASSERT_EQUAL_UINT8(0b00100000, reg->f);
}

TEST(alu, inc_ShouldSetZero) {
    reg->f = 0b00000000;

    uint8_t val = alu_inc8(reg, 255);

    TEST_ASSERT_EQUAL_UINT8(0, val);
    TEST_ASSERT_EQUAL_UINT8(0b10100000, reg->f);
}

TEST_GROUP_RUNNER(alu) {

    // ADD
    RUN_TEST_CASE(alu, alu_add8_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, alu_add8_ShouldSetCarry);
    RUN_TEST_CASE(alu, alu_add16_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, alu_add16_ShouldSetCarry);

    // INC
    RUN_TEST_CASE(alu, inc_ShouldIncReg);
    RUN_TEST_CASE(alu, inc_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, inc_ShouldSetZero);
}