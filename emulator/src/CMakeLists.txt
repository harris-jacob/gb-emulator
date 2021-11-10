
add_executable(emulator
   run.c 
)


# Copying in our e2e instruction test roms
file(GLOB INPUT ../gb-test-roms/cpu_instrs/individual/*.gb)
file(MAKE_DIRECTORY ${PROJECT_BINARY_DIR}/roms)
file(COPY ${INPUT} DESTINATION ${PROJECT_BINARY_DIR}/roms)

target_link_libraries(emulator PRIVATE
gb-lib
)
