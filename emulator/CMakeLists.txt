# adapted from http://www.throwtheswitch.org/build/cmake
cmake_minimum_required(VERSION 3.13.0)
project("emulator"
        DESCRIPTION "Game boy emulator written for fun"
        LANGUAGES C) 

#
set(CMAKE_C_STANDARD 11)

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)

set(EXECUTABLE_OUTPUT_PATH ${PROJECT_BINARY_DIR}/bin)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -s MIN_WEBGL_VERSION=2 -s")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -s =2 -s")


# Modules that exist in both our release and our unit tests, are added now
add_subdirectory(gb-lib)
add_subdirectory(src) 

if(${CMAKE_BUILD_TYPE} STREQUAL "Debug")
# If we had any files that ONLY were compiled into our release, for example startup code,
  # it's really handy to put it here. 
elseif(${CMAKE_BUILD_TYPE} STREQUAL "Test")

  # It also needs to understand we have code in source and test directories 
  set(UNITY_EXTENSION_FIXTURE ON CACHE BOOL "" FORCE)
  add_subdirectory(Unity) 
  add_subdirectory(test) 
endif()
