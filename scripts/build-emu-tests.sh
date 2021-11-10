#!/bin/bash
# this script builds unit tests
cmake ./emulator -UTARGET_GROUP -DTARGET_GROUP=test -B ./emulator/build
cmake --build ./emulator/build/test
