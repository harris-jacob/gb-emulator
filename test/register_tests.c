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

/* The MMU destroy func should destroy an MMU instance */
TEST(registers, register_destroy_ShouldDestroyRegister) {
    reg_destroy(&reg);
    TEST_ASSERT_NULL(reg);
}

TEST_GROUP_RUNNER(registers) {
    RUN_TEST_CASE(registers, register_create_ShouldCreateRegister);
    RUN_TEST_CASE(registers, register_destroy_ShouldDestroyRegister);
}


