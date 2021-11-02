#!/bin/bash
set -e

export BUILD_TYPE=$1

# build emulator
cd emulator
mkdir -p build 
cd build
# emcmake replaces gcc in make files with emcc. Also sets the appropriate env vars
emcmake cmake ..\
    -DCMAKE_BUILD_TYPE=${BUILD_TYPE}

# Generate wasm object files.
emmake make

# find object files and the static libraries
find ./src/ -name "*.c.o" -exec cp '{}' ./ \;
find ./gb-lib/ -name "*.a" -exec cp '{}' ./ \;

# # run emcc to handle linking and generate js/wasm
emcc run.c.o libgb-lib.a -o bin/emulator.js -s MODULARIZE -s EXPORTED_RUNTIME_METHODS=ccall \
-s ENVIRONMENT=web

cd ../..

# copy built files to binding point
 mkdir -p ./ui/src/static
 find ./emulator/build/bin/ -name "*.wasm" -exec cp '{}' ./ui/src/static/ \;
 mkdir -p ui/src/emulator
 find ./emulator/build/bin/ -name "emulator.js" -exec cp '{}' ./ui/src/emulator/ \;
 find ./ui/src/emulator -name "emulator.js" -exec sed -i '1i/* eslint-disable */' '{}' \;