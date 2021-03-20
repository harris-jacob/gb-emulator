#!/bin/bash

cmake . -UTARGET_GROUP -DTARGET_GROUP=test
cmake --build test
