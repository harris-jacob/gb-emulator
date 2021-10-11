#find_package(SDL2 REQUIRED)

add_library(emulator
   # emu.c
    mmu.c
    cpu.c
    rom.c
    register.c
    opcodes.c
    cb_opcodes.c
    ppu.c
)

target_include_directories(emulator PUBLIC .)

# add_executable(main main.c)


#target_include_directories(main PUBLIC . ${SDL2_INCLUDE_DIRS})
#target_link_libraries(main PUBLIC emulator ${SDL2_LIBRARIES})
