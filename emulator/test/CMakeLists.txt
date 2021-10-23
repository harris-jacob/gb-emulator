# project test runner setup
add_executable(test 
  runner.c 
  mmu_tests.c 
  register_tests.c 
  alu_op_tests.c  
  bit_op_tests.c
  rotation_op_tests.c
  stack_tests.c
  cpu_tests.c)

target_include_directories(test PUBLIC
  ${mmu_SOURCE_DIR}
  ${emulator_SOURCE_DIR}
  ${unity_PUBLIC_HEADERS}
  .
)

target_link_libraries(test PUBLIC emulator unity)

add_test(test test)