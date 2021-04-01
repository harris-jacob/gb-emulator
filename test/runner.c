#include "unity_fixture.h"

static void RunAllTests(void) {
    RUN_TEST_GROUP(mmu);
    RUN_TEST_GROUP(registers);
    RUN_TEST_GROUP(alu);
}

int main(int argc, const char* argv[]) {
    return UnityMain(argc, argv, RunAllTests);
};
