#!/bin/bash

cmake . -UTARGET_GROUP -DTARGET_GROUP=test -B ./build
cmake --build ./build/test
