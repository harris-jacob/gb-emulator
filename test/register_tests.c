#include "unity.h"
#include "register.h"
#include "unity_fixture.h"

reg_t* reg;

// Register group
TEST_GROUP(registers);

// Runs before each test
TEST_SETUP(registers) {
    reg = reg_create();
}

// Runs after each test
TEST_TEAR_DOWN(registers) {
    if(reg != NULL) {
        reg_destroy(&reg);
    }
}

/* The register create func should create a register instance */
TEST(registers, register_create_ShouldCreateRegister) {
    TEST_ASSERT_NOT_NULL(reg);
}

/* The register destroy func should destroy an MMU instance */
TEST(registers, register_destroy_ShouldDestroyRegister) {
    reg_destroy(&reg);
    TEST_ASSERT_NULL(reg);
}

TEST(registers, get_carry_ShouldGetBitVal) {
    reg->f = 16;
    TEST_ASSERT_EQUAL_UINT8(1, get_carry(reg));
} 

TEST(registers, get_halfcarry_ShouldGetBitVal) {
    reg->f = 32;
    TEST_ASSERT_EQUAL_UINT8(1, get_halfcarry(reg));
} 

TEST(registers, get_subtract_ShouldGetBitVal) {
    reg->f = 64;
    TEST_ASSERT_EQUAL_UINT8(1, get_subtract(reg));
} 

TEST(registers, get_zero_ShouldGetBitVal) {
    reg->f = 128;
    TEST_ASSERT_EQUAL_UINT8(1, get_zero(reg));
} 


TEST(registers, set_carry_ShouldSetBitVal) {
    reg->f = 1;
    set_carry(reg);

    TEST_ASSERT_EQUAL_UINT8(17, reg->f);
} 

TEST(registers, set_halfcarry_ShouldSetBitVal) {
    reg->f = 2;
    set_halfcarry(reg);
    TEST_ASSERT_EQUAL_UINT8(34, reg->f);
} 

TEST(registers, set_subtract_ShouldSetBitVal) {
    reg->f = 2;
    set_subtract(reg);
    TEST_ASSERT_EQUAL_UINT8(66, reg->f);
} 

TEST(registers, set_zero_ShouldSetBitVal) {
    reg->f = 2;
    set_zero(reg);
    TEST_ASSERT_EQUAL_UINT8(130, reg->f);
}

TEST(registers, reset_carry_ShouldResetBitVal) {
    reg->f = 17;
    reset_carry(reg);

    TEST_ASSERT_EQUAL_UINT8(1, reg->f);
} 

TEST(registers, reset_halfcarry_ShouldResetBitVal) {
    reg->f = 49;
    reset_halfcarry(reg);
    TEST_ASSERT_EQUAL_UINT8(17, reg->f);
} 

TEST(registers, reset_subtract_ShouldResetBitVal) {
    reg->f = 64;
    reset_subtract(reg);
    TEST_ASSERT_EQUAL_UINT8(0, reg->f);
} 

TEST(registers, reset_zero_ShouldResetBitVal) {
    reg->f = 131;
    reset_zero(reg);
    TEST_ASSERT_EQUAL_UINT8(3, reg->f);
} 

TEST(registers, should_halfcarry8_ShouldAlertHalfCarry) {
    uint8_t a = 0b0001010;
    uint8_t b = 0b0001100;

    TEST_ASSERT_TRUE(should_halfcarry8(a, b));

}

TEST(registers, should_halfcarry8_ShouldReturnFalseIfNoCarry) {
    uint8_t a = 0b0000010;
    uint8_t b = 0b0001010;

    TEST_ASSERT_FALSE(should_halfcarry8(a, b));
}

TEST(registers, should_carry8_ShouldAlertCarry) {
    uint8_t a = 0b010000000;
    uint8_t b = 0b011000000;

    TEST_ASSERT_TRUE(should_carry8(a, b));
}


TEST(registers, should_carry8_ShouldReturnFalseIfNoCarry) {
    uint8_t a = 0b011000000;
    uint8_t b = 0b000100000;

    TEST_ASSERT_FALSE(should_carry8(a, b));
}

TEST(registers, should_halfcarry16_ShouldAlertHalfCarry) {
    uint8_t a = 0b010000000;
    uint8_t b = 0b011000000;
    
    TEST_ASSERT_TRUE(should_halfcarry16(a, b));

}

TEST(registers, should_halfcarry16_ShouldReturnFalseIfNoCarry) {
    uint8_t a = 0b011000000;
    uint8_t b = 0b000100000;

    TEST_ASSERT_FALSE(should_halfcarry16(a, b));
}

TEST(registers, should_carry16_ShouldAlertCarry) {
    uint16_t a = 0xfff1;
    uint16_t b = 0xffff;

    TEST_ASSERT_TRUE(should_carry16(a, b));
}


TEST(registers, should_carry16_ShouldReturnFalseIfNoCarry) {
    uint16_t a = 0xff;
    uint16_t b = 0xfff;

    TEST_ASSERT_FALSE(should_carry16(a, b));
}

TEST(registers, alu_add8_ShouldSetHalfCarry) {
    reg->f = 0b10000000;
    uint8_t val = alu_add8(reg, 11, 5);

    TEST_ASSERT_EQUAL_UINT8(16, val);
    TEST_ASSERT_EQUAL_UINT8(0b00100000, reg->f);
    
}

TEST(registers, alu_add8_ShouldSetCarry) {
    reg->f = 0b11000000;
    uint8_t val = alu_add8(reg, 0xff, 128);

    TEST_ASSERT_EQUAL_UINT8(127, val);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
    
}

TEST(registers, alu_add16_ShouldSetHalfCarry) {
    reg->f = 0b10000000;
    uint16_t val = alu_add16(reg, 0xff, 10);

    TEST_ASSERT_EQUAL_UINT16(265, val);
    TEST_ASSERT_EQUAL_UINT8(160, reg->f);

}

TEST(registers, alu_add16_ShouldSetCarry) {
    reg->f = 0b00000000;
    uint16_t val = alu_add16(reg, 0b01100000000000000, 0b001000000000000000);

    TEST_ASSERT_EQUAL_UINT16(16384, val);
    TEST_ASSERT_EQUAL_UINT16(0b00010000, reg->f);
}


TEST_GROUP_RUNNER(registers) {
    RUN_TEST_CASE(registers, register_create_ShouldCreateRegister);
    RUN_TEST_CASE(registers, register_destroy_ShouldDestroyRegister);
    
    // GETTERS
    RUN_TEST_CASE(registers, get_carry_ShouldGetBitVal);
    RUN_TEST_CASE(registers, get_halfcarry_ShouldGetBitVal);
    RUN_TEST_CASE(registers, get_subtract_ShouldGetBitVal);
    RUN_TEST_CASE(registers, get_zero_ShouldGetBitVal);

    // SETTERS
    RUN_TEST_CASE(registers, set_carry_ShouldSetBitVal);
    RUN_TEST_CASE(registers, set_halfcarry_ShouldSetBitVal);
    RUN_TEST_CASE(registers, set_subtract_ShouldSetBitVal);
    RUN_TEST_CASE(registers, set_zero_ShouldSetBitVal);

    // RESETTERS
    RUN_TEST_CASE(registers, reset_carry_ShouldResetBitVal);
    RUN_TEST_CASE(registers, reset_halfcarry_ShouldResetBitVal);
    RUN_TEST_CASE(registers, reset_subtract_ShouldResetBitVal);
    RUN_TEST_CASE(registers, reset_zero_ShouldResetBitVal);


    // CARRY CHECKERS
    RUN_TEST_CASE(registers, should_halfcarry8_ShouldAlertHalfCarry);
    RUN_TEST_CASE(registers, should_halfcarry8_ShouldReturnFalseIfNoCarry);
    RUN_TEST_CASE(registers, should_carry8_ShouldAlertCarry);
    RUN_TEST_CASE(registers, should_carry8_ShouldReturnFalseIfNoCarry);
    RUN_TEST_CASE(registers, should_halfcarry16_ShouldAlertHalfCarry);
    RUN_TEST_CASE(registers, should_halfcarry16_ShouldReturnFalseIfNoCarry);
    RUN_TEST_CASE(registers, should_carry16_ShouldAlertCarry);
    RUN_TEST_CASE(registers, should_carry16_ShouldReturnFalseIfNoCarry);


    // ALU OPS
    RUN_TEST_CASE(registers, alu_add8_ShouldSetHalfCarry);
    RUN_TEST_CASE(registers, alu_add8_ShouldSetCarry);
    RUN_TEST_CASE(registers, alu_add16_ShouldSetHalfCarry);
    RUN_TEST_CASE(registers, alu_add16_ShouldSetCarry);
}


