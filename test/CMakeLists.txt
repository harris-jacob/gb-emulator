add_executable(mmu_test .c)
 
target_link_libraries(mmu_test src unity)
 
add_test(mmu_test )