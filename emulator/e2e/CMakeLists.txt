
add_executable(e2e
   run.c 
)

# Copying in our e2e instruction test roms
file(GLOB INPUT ../gb-test-roms/cpu_instrs/individual/*.gb)

# Copying in our experimental test roms. TETRIS!!!!!!!
file(GLOB INPUT ../roms/*.gb)

file(COPY ${INPUT} DESTINATION ${CMAKE_CURRENT_BINARY_DIR})

target_include_directories(e2e PRIVATE
  ${emulator_SOURCE_DIR}
)

target_link_libraries(e2e PUBLIC 
emulator
)
