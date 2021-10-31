
add_executable(emulator
   run.c 
)

# Copying in our e2e instruction test roms
file(GLOB INPUT ../gb-test-roms/cpu_instrs/individual/*.gb)

# Copying in our experimental test roms. TETRIS!!!!!!!
file(GLOB INPUT ../roms/*.gb)

file(COPY ${INPUT} DESTINATION ${CMAKE_CURRENT_BINARY_DIR})

add_compile_options("-s MODULARIZE -s EXPORTED_RUNTIME_METHOD=ccall ")

target_link_libraries(emulator PRIVATE
gb-lib
)
