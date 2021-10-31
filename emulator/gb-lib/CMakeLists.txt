#find_package(SDL2 REQUIRED)

add_library(gb-lib
   # emu.c
    mmu.c
    cpu.c
    rom.c
    register.c
    opcodes.c
    cb_opcodes.c
    ppu.c
)

target_include_directories(gb-lib PUBLIC .)