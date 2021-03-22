add_library(mmu
    mmu.c
)

add_library(register
    register.c
)

target_include_directories(mmu PUBLIC
    .
)

target_include_directories(register PUBLIC
    .
)
