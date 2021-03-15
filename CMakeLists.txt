cmake_minimum_required(VERSION 3.17.2)
project(emulator
    VERSION 0.1.0
    DESCRIPTION "Game Boy emulator"
    LANGUAGE C
)

set(CMAKE_C_STANDARD 17)

add_subdirectory(src)