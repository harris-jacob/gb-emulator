# adapted from http://www.throwtheswitch.org/build/cmake
project("emulator" C) 
cmake_minimum_required(VERSION 3.0)

#
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_BINARY_DIR}/bin)

# We declare a variable to select what to build set
set(TARGET_GROUP test CACHE STRING "What Should I Build?")

# Modules that exist in both our release and our unit tests, are added now
  add_subdirectory(src) 

if(TARGET_GROUP STREQUAL production)
# If we had any files that ONLY were compiled into our release, for example startup code,
  # it's really handy to put it here. 
elseif(TARGET_GROUP STREQUAL test)

  # It also needs to understand we have code in source and test directories 
  add_subdirectory(unity) 
  add_subdirectory(test) 
else() 
  message(FATAL_ERROR "I don't know the TARGET_GROUP you gave me!")
endif()