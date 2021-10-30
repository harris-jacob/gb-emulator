#!/bin/bash
set -e

export BUILD_TYPE=$1

# build emulator
cd emulator
mkdir -p build 
cd build
cmake ..\
    -DCMAKE_BUILD_TYPE=${BUILD_TYPE} \
    -DCMAKE_TOOLCHAIN_FILE=../toolchains/generic/Emscripten-wasm.cmake \
    -DCMAKE_PREFIX_PATH=~/emsdk/upstream/emscripten/system  

cmake --build . 