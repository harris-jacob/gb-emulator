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

target_link_libraries(cpu PUBLIC mmu register
)

target_include_directories(mmu PUBLIC
    .
)

target_include_directories(cpu PUBLIC .)

target_include_directories(register PUBLIC
    .
)
