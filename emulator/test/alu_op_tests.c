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

TEST(alu, alu_add8_ShouldAdd) {
    reg->f = 0b01000000;
    reg->a = 5;
    alu_add8(reg, 5);

    TEST_ASSERT_EQUAL_UINT8(10, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00000000, reg->f);
    
}

TEST(alu, alu_add8_ShouldSetHalfCarry) {
    reg->f = 0b10000000;
    reg->a = 11;
    alu_add8(reg, 5);

    TEST_ASSERT_EQUAL_UINT8(16, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00100000, reg->f);
    
}

TEST(alu, alu_add8_ShouldSetCarry) {
    reg->f = 0b11000000;
    reg->a = 0xff;
    alu_add8(reg, 128);

    TEST_ASSERT_EQUAL_UINT8(127, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
    
}

TEST(alu, alu_add8_ShouldSetZero) {
    reg->f = 0b00000000;
    reg->a = 0;
    alu_add8(reg, 0);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b10000000, reg->f);
}

TEST(alu, alu_add16_ShouldAdd) {
    reg->f = 0b00000000;
    uint16_t val = alu_add16(reg, 120, 100);

    TEST_ASSERT_EQUAL_UINT16(val, 220);
    TEST_ASSERT_EQUAL_UINT8(0b00000000, reg->f);
}

TEST(alu, alu_add16_ShouldSetHalfCarry) {
    reg->f = 0b00000000;
    uint16_t val = alu_add16(reg, 255, 10);

    TEST_ASSERT_EQUAL_UINT16(265, val);
    TEST_ASSERT_EQUAL_UINT8(0b00100000, reg->f);
}

TEST(alu, alu_add16_ShouldSetCarry) {
    reg->f = 0b00000000;
    uint16_t val = alu_add16(reg, 0b01100000000000000, 0b001000000000000000);

    TEST_ASSERT_EQUAL_UINT16(16384, val);
    TEST_ASSERT_EQUAL_UINT16(16, reg->f);
}

TEST(alu, alu_add16_ShouldNotSetZero) {
    reg->f = 0b00000000;
    uint16_t val = alu_add16(reg, 0, 0);

    TEST_ASSERT_EQUAL_UINT16(0, val);
    TEST_ASSERT_EQUAL_UINT8(0b000000000, reg->f);
}

TEST(alu, alu_sub8_ShouldSubtract) {
    reg->f = 0b00000000;
    reg->a = 8;
    alu_subtract8(reg, 2);

    TEST_ASSERT_EQUAL_UINT16(6, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b01000000, reg->f);
}

TEST(alu, alu_sub8_ShouldSetHalfCarry) {
    reg->f = 0b00000000;
    reg->a = 16;

    alu_subtract8(reg, 2);

    TEST_ASSERT_EQUAL_UINT8(14, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b01100000, reg->f);
}

TEST(alu, alu_sub8_ShouldSetCarry) {
    reg->f = 0b00000000;
    reg->a = 0b00100000;
    alu_subtract8(reg, 0b11000000);

    TEST_ASSERT_EQUAL_UINT8(96, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b01010000, reg->f);
}

TEST(alu, alu_sub8_ShouldSetZero) {
    reg->f = 0b00000000;
    reg->a = 10;
    alu_subtract8(reg, 10);
    
    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b11000000, reg->f);
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


TEST(alu, dec_ShouldDecReg) {
    reg->f = 0b00000000;

    uint8_t val = alu_dec8(reg, 10);

    TEST_ASSERT_EQUAL_UINT8(9, val);
    TEST_ASSERT_EQUAL_UINT8(0b01000000,reg->f);
}

TEST(alu, dec_ShouldSetHalfCarry) {
    reg->f = 0b00000000;

    uint8_t val = alu_dec8(reg, 16);

    TEST_ASSERT_EQUAL_UINT8(15, val);
    TEST_ASSERT_EQUAL_UINT8(0b01100000, reg->f);
}

TEST(alu, dec_ShouldSetZero) {
    reg->f = 0b00000000;

    uint8_t val = alu_dec8(reg, 1);

    TEST_ASSERT_EQUAL_UINT8(0, val);
    TEST_ASSERT_EQUAL_UINT8(0b11000000, reg->f);
}

TEST(alu, adc_ShouldEqualAddIfNoCarrySet) {
    reg->f = 0b00000000;
    reg->a = 5;

    alu_adc8(reg, 5);
    
    TEST_ASSERT_EQUAL(10, reg->a);
    TEST_ASSERT_EQUAL(0, reg->f);
}

TEST(alu, adc_ShouldAddCarryIfSet) {
    reg->f = 0b00010000;
    reg->a = 5;

    alu_adc8(reg, 4);
    
    TEST_ASSERT_EQUAL(10, reg->a);
    TEST_ASSERT_EQUAL(0, reg->f);
}

TEST(alu, adc_ShouldSetHalfCarry) {
    reg->f = 0b00010000;
    reg->a = 5;

    alu_adc8(reg, 10);
    
    TEST_ASSERT_EQUAL(16, reg->a);
    TEST_ASSERT_EQUAL(32, reg->f);
}

TEST(alu, adc_ShouldSetCarry) {
    reg->f = 0b00000000;
    reg->a = 255;

    alu_adc8(reg, 64);
    
    TEST_ASSERT_EQUAL(63, reg->a);
    TEST_ASSERT_EQUAL(16, reg->f);
}

TEST(alu, adc_ShouldSetZero) {
    reg->f = 0b00010000;
    reg->a = 255;

    alu_adc8(reg, 0);
    
    TEST_ASSERT_EQUAL(0, reg->a);
    TEST_ASSERT_EQUAL(176, reg->f);
}

TEST(alu, sbc_ShouldEqualSubIfNoCarrySet) {
    reg->f = 0b00000000;
    reg->a = 10;

    alu_sbc8(reg, 5);
    
    TEST_ASSERT_EQUAL(5, reg->a);
    // SUB set
    TEST_ASSERT_EQUAL(64, reg->f);
}

TEST(alu, sbc_ShouldSubCarryIfSet) {
    reg->f = 0b00010000;
    reg->a = 6;

    alu_sbc8(reg, 4);
    
    TEST_ASSERT_EQUAL(1, reg->a);
    TEST_ASSERT_EQUAL(64, reg->f);
}

TEST(alu, sbc_ShouldSetHalfCarry) {
    reg->f = 0b00010000;
    reg->a = 48;

    alu_sbc8(reg, 7);
    
    TEST_ASSERT_EQUAL(40, reg->a);
    TEST_ASSERT_EQUAL(96, reg->f);
}

TEST(alu, sbc_ShouldSetCarry) {
    reg->f = 0b00000000;
    reg->a = 128;

    alu_sbc8(reg, 192);
    
    TEST_ASSERT_EQUAL(192, reg->a);
    TEST_ASSERT_EQUAL(80, reg->f);
}

TEST(alu, sbc_ShouldSetZero) {
    reg->f = 0b00010000;
    reg->a = 5;

    alu_sbc8(reg, 4);
    
    TEST_ASSERT_EQUAL(0, reg->a);
    TEST_ASSERT_EQUAL(192, reg->f);
}

TEST(alu, cp_ShouldSetZeroOnEqual) {
    reg->a = 5;
    
    cp(reg, 5);

    TEST_ASSERT_EQUAL(0b11000000, reg->f);
}

TEST(alu, cp_ShouldSetHalfCarry) {
    reg->a = 0b00100111;
    uint8_t val = 0b00001100;
    
    cp(reg, val);

    TEST_ASSERT_EQUAL(0b01100000, reg->f);
}

TEST(alu, cp_ShouldSetCarry) {
    reg->a = 0b01001101;
    uint8_t val = 0b11000001;
    
    cp(reg, val);

    TEST_ASSERT_EQUAL(0b01010000, reg->f);
}

TEST_GROUP_RUNNER(alu) {

    // ADD 8
    RUN_TEST_CASE(alu, alu_add8_ShouldAdd);
    RUN_TEST_CASE(alu, alu_add8_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, alu_add8_ShouldSetCarry);
    RUN_TEST_CASE(alu, alu_add8_ShouldSetZero);

    // ADD 16
    RUN_TEST_CASE(alu, alu_add16_ShouldAdd);
    RUN_TEST_CASE(alu, alu_add16_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, alu_add16_ShouldSetCarry);
    RUN_TEST_CASE(alu, alu_add16_ShouldNotSetZero);

    // Sub
    RUN_TEST_CASE(alu, alu_sub8_ShouldSubtract);
    RUN_TEST_CASE(alu, alu_sub8_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, alu_sub8_ShouldSetCarry);
    RUN_TEST_CASE(alu, alu_sub8_ShouldSetZero);

    // INC
    RUN_TEST_CASE(alu, inc_ShouldIncReg);
    RUN_TEST_CASE(alu, inc_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, inc_ShouldSetZero);

    // DEC
    RUN_TEST_CASE(alu, dec_ShouldDecReg);
    RUN_TEST_CASE(alu, dec_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, dec_ShouldSetZero);

    // ADC
    RUN_TEST_CASE(alu, adc_ShouldEqualAddIfNoCarrySet);
    RUN_TEST_CASE(alu, adc_ShouldAddCarryIfSet);
    RUN_TEST_CASE(alu, adc_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, adc_ShouldSetCarry);
    RUN_TEST_CASE(alu, adc_ShouldSetZero);

    // SBC
    RUN_TEST_CASE(alu, sbc_ShouldEqualSubIfNoCarrySet);
    RUN_TEST_CASE(alu, sbc_ShouldSubCarryIfSet);
    RUN_TEST_CASE(alu, sbc_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, sbc_ShouldSetCarry);
    RUN_TEST_CASE(alu, sbc_ShouldSetZero);

    // CP
    RUN_TEST_CASE(alu, cp_ShouldSetZeroOnEqual);
    RUN_TEST_CASE(alu, cp_ShouldSetHalfCarry);
    RUN_TEST_CASE(alu, cp_ShouldSetCarry);

}