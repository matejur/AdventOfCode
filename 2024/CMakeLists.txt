# Minimum required version of CMake
cmake_minimum_required(VERSION 3.10)

# Name of the project
project(AdventOfCode)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Include all source files
file(GLOB SOURCES
    main.cpp
    solutions/*.cpp
    utils.cpp
)

# Add the executable target
add_executable(advent_of_code ${SOURCES})