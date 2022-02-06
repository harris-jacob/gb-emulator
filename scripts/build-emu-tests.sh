#!/bin/bash
# this script builds unit tests
cmake ./emulator -DCMAKE_BUILD_TYPE=Test -B ./emulator/build
cmake --build ./emulator/build/test
