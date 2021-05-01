find_package(SDL2 REQUIRED)

add_library(emulator
    emu.c
    mmu.c
    cpu.c
    rom.c
    register.c
    opcodes.c
    lcd.c
)

target_include_directories(emulator PUBLIC . ${SDL2_INCLUDE_DIRS})

target_link_libraries(emulator PUBLIC ${SDL2_LIBRARIES})
