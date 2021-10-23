#include "unity_fixture.h"

static void RunAllTests(void) {
    RUN_TEST_GROUP(mmu);
    RUN_TEST_GROUP(registers);
    RUN_TEST_GROUP(alu);
    RUN_TEST_GROUP(rot);
    RUN_TEST_GROUP(cpu);
    RUN_TEST_GROUP(bit);
    RUN_TEST_GROUP(stack);
}

int main(int argc, const char* argv[]) {
    return UnityMain(argc, argv, RunAllTests);
};
