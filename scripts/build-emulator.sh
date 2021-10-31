#!/bin/bash
set -e

export BUILD_TYPE=$1

# build emulator
cd emulator
mkdir -p build 
cd build
emcmake ..\
    -DCMAKE_BUILD_TYPE=${BUILD_TYPE} \
    -DCMAKE_PREFIX_PATH=~/emsdk/upstream/emscripten/system  \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

cmake --build . 

cd ../..

# copy built files to binding point
find ./emulator/build/bin/ -name "*.wasm" -exec cp '{}' ./ui/src/assets/ \;
mkdir -p ui/src/emulator
find ./emulator/build/bin/ -name "emulator.js" -exec cp '{}' ./ui/src/emulator/ \;
find ./ui/src/emulator -name "emulator.js" -exec sed -i '1i/* eslint-disable */' '{}' \;