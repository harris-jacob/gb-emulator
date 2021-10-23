#include "unity.h"
#include "cpu.h"
#include "unity_fixture.h"


cpu_t* cpu;
mmu_t* mmu;

// Register group
TEST_GROUP(stack);

// Runs before each test
TEST_SETUP(stack) {
    mmu = mmu_create(); 
    cpu = cpu_create(mmu); 
}

TEST_TEAR_DOWN(stack) {
    if(cpu != NULL) {
        cpu_destroy(&cpu);
    }
}

TEST(stack, push_ShouldMoveTheStackPtr) {
    cpu->reg->sp = 22;
    stack_push(cpu, 0x12);

    TEST_ASSERT_EQUAL_UINT8(20, cpu->reg->sp);
}

TEST(stack, push_ShouldWriteToStackSegment) {
    cpu->reg->sp = 22;
    stack_push(cpu, 0x12);

    TEST_ASSERT_EQUAL_UINT8(0x12, mmu_read_addr16(cpu->mmu, cpu->reg->sp));
}

TEST(stack, pop_ShouldMoveTheStackPtr) {
    cpu->reg->sp = 22;
    stack_pop(cpu);

    TEST_ASSERT_EQUAL_UINT8(24, cpu->reg->sp);
}

TEST(stack, pop_ShouldReadFromStackSegment) {
    cpu->reg->sp = 22;
    cpu->mmu->addr[22] = 30;

    uint8_t val = stack_pop(cpu);

    TEST_ASSERT_EQUAL_UINT8(30, val);
}

TEST_GROUP_RUNNER(stack) {

    // PUSH
    RUN_TEST_CASE(stack, push_ShouldMoveTheStackPtr);
    RUN_TEST_CASE(stack, push_ShouldWriteToStackSegment);

    // POP
    RUN_TEST_CASE(stack, pop_ShouldMoveTheStackPtr);
    RUN_TEST_CASE(stack, pop_ShouldReadFromStackSegment);
}
