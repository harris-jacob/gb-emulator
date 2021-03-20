add_executable(test mmu_test.c)


target_include_directories(test PRIVATE
  ${mmu_SOURCE_DIR}
)

target_link_libraries(test PUBLIC mmu unity)

add_test(NAME test COMMAND test)