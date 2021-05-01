add_library(mmu
    mmu.c
)

add_library(register
    register.c
)

add_library(
    cpu
    cpu.c
    opcodes.c
)

add_library(
    rom
    rom.c
)

add_library(
    lcd
    lcd.c
)

target_include_directories(lcd PUBLIC .)

target_link_libraries(lcd PUBLIC mmu)

target_include_directories(rom PUBLIC .)

target_link_libraries(rom PUBLIC mmu)

target_link_libraries(cpu PUBLIC mmu register
)

target_include_directories(mmu PUBLIC
    .
)

target_include_directories(cpu PUBLIC .)

target_include_directories(register PUBLIC
    .
)
