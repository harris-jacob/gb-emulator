
add_executable(e2e
   run.c 
)

file(GLOB INPUT ../gb-test-roms/cpu_instrs/individual/*.gb)
file(COPY ${INPUT} DESTINATION ${CMAKE_CURRENT_BINARY_DIR})

target_include_directories(e2e PRIVATE
  ${cpu_SOURCE_DIR}
)

target_link_libraries(e2e PUBLIC 
cpu
)