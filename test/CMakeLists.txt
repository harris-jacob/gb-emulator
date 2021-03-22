# project test runner setup
add_executable(test runner.c mmu_tests.c register_tests.c)

target_include_directories(test PUBLIC
  ${mmu_SOURCE_DIR}
  ${register_SOURCE_DIR}
  ${unity_PUBLIC_HEADERS}
  .
)

target_link_libraries(test PUBLIC mmu register unity)

add_test(test test)