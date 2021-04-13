#include "cpu.h"
#include "unity.h"
#include "unity_fixture.h"

cpu_t* cpu;

// Declare group
TEST_GROUP(cpu);

// Runs before each test
TEST_SETUP(cpu) {
    cpu = cpu_create();
}

// Runs after each test
TEST_TEAR_DOWN(cpu) {
    if(cpu != NULL) {
        cpu_destroy(&cpu);
    }
}

/* The cpu create func should create an cpu instance */
TEST(cpu, cpu_create_ShouldCreateCPU) {
    TEST_ASSERT_NOT_NULL(cpu);
}

/* The CPU destroy func should destroy an CPU instance */
TEST(cpu, cpu_destroy_ShouldDestroyCPU) {
    cpu_destroy(&cpu);
    TEST_ASSERT_NULL(cpu);
}


TEST_GROUP_RUNNER(cpu) {
    RUN_TEST_CASE(cpu, cpu_create_ShouldCreateCPU);
    RUN_TEST_CASE(cpu, cpu_destroy_ShouldDestroyCPU);
}