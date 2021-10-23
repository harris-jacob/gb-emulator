#include "unity.h"
#include "register.h"
#include "unity_fixture.h"

reg_t* reg;

// Register group
TEST_GROUP(rot);

// Runs before each test
TEST_SETUP(rot) {
    reg = reg_create();
}

// Runs after each test
TEST_TEAR_DOWN(rot) {
    if(reg != NULL) {
        reg_destroy(&reg);
    }
}

TEST(rot, rot_rr_ShouldRotateBitsThroughCarry) {
    reg->f = 0b10010000;
    reg->a = 0b11001010;
    reg->a = rr(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b11100101, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0, reg->f);
}

TEST(rot, rot_rr_ShouldSetZeroAndCarry) {
    reg->f = 0b00000000;
    reg->a = 0b00000001;
    reg->a = rr(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(144, reg->f);
}

TEST(rot, rot_rl_ShouldRotateBitsThroughCarry) {
    reg->f = 0b10010000;
    reg->a = 0b01001010;
    reg->a = rl(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b10010101, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0, reg->f);
}

TEST(rot, rot_rl_ShouldSetZeroAndCarry) {
    reg->f = 0b00000000;
    reg->a = 0b10000000;
    reg->a = rl(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(144, reg->f);
}

TEST(rot, rot_rrc_ShouldRotateBitsAndCopyToCarry) {
    reg->f = 0b10000000;
    reg->a = 0b01001011;
    reg->a = rrc(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b10100101, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
}


TEST(rot, rot_rrc_ShouldSetZero) {
    reg->f = 0b01100000;
    reg->a = 0b00000000;
    reg->a = rrc(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b10000000, reg->f);
}

TEST(rot, rot_rlc_ShouldRotateBitsAndCopyToCarry) {
    reg->f = 0b00000000;
    reg->a = 0b10001011;
    reg->a = rlc(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b00010111, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
}

TEST(rot, rot_rlc_ShouldSetZero) {
    reg->f = 0b01100000;
    reg->a = 0b00000000;
    reg->a = rlc(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b10000000, reg->f);
}

TEST(rot, rot_rra_ShouldRotateBitsAndCopyToCarry) {
    reg->f = 0b00000000;
    reg->a = 0b10001011;
    reg->a = rlc(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b00010111, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
}

TEST(rot, rot_sra_ShouldArithmeticShiftRight) {
    reg->f = 0b00000000;
    reg->a = 0b10110001;
    reg->a = sra(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b11011000, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
}

TEST(rot, rot_sra_ShouldSetZero) {
    reg->f = 0b01100000;
    reg->a = 0b00000001;
    reg->a = sra(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b10010000, reg->f);
}

TEST(rot, rot_sla_ShouldArithmeticShiftLeft) {
    reg->f = 0b00000000;
    reg->a = 0b10110001;
    reg->a = sla(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b01100010, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
}

TEST(rot, rot_sla_ShouldSetZero) {
    reg->f = 0b01100000;
    reg->a = 0b10000000;
    reg->a = sla(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b10010000, reg->f);
}

TEST(rot, rot_srl_ShouldLogicalShiftRight) {
    reg->f = 0b00000000;
    reg->a = 0b10110001;
    reg->a = srl(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0b01011000, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b00010000, reg->f);
}

TEST(rot, rot_srl_ShouldSetZero) {
    reg->f = 0b01100000;
    reg->a = 0b00000001;
    reg->a = srl(reg, reg->a);

    TEST_ASSERT_EQUAL_UINT8(0, reg->a);
    TEST_ASSERT_EQUAL_UINT8(0b10010000, reg->f);
}

TEST_GROUP_RUNNER(rot) {

    // RR
    RUN_TEST_CASE(rot, rot_rr_ShouldRotateBitsThroughCarry);
    RUN_TEST_CASE(rot, rot_rr_ShouldSetZeroAndCarry);

    // LR
    RUN_TEST_CASE(rot, rot_rl_ShouldRotateBitsThroughCarry);
    RUN_TEST_CASE(rot, rot_rl_ShouldSetZeroAndCarry);

    // RRC
    RUN_TEST_CASE(rot, rot_rrc_ShouldRotateBitsAndCopyToCarry);
    RUN_TEST_CASE(rot, rot_rrc_ShouldSetZero);

    // RLC
    RUN_TEST_CASE(rot, rot_rlc_ShouldRotateBitsAndCopyToCarry);
    RUN_TEST_CASE(rot, rot_rlc_ShouldSetZero);

    // SRA
    RUN_TEST_CASE(rot, rot_sra_ShouldArithmeticShiftRight);
    RUN_TEST_CASE(rot, rot_sra_ShouldSetZero);

    // SLA
    RUN_TEST_CASE(rot, rot_sla_ShouldArithmeticShiftLeft);
    RUN_TEST_CASE(rot, rot_sla_ShouldSetZero);

    // SRL
    RUN_TEST_CASE(rot, rot_srl_ShouldLogicalShiftRight);
    RUN_TEST_CASE(rot, rot_srl_ShouldSetZero);
}
