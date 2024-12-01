cmake_minimum_required(VERSION 3.14)
project(advent_of_code_rhyn0_cpp)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Set using clang
set(CMAKE_C_COMPILER /usr/bin/clang)
set(CMAKE_CXX_COMPILER /usr/bin/clang++)


# Automatically find all solution source files
file(GLOB_RECURSE SOLUTION_SOURCES 
    "${CMAKE_CURRENT_SOURCE_DIR}/src/solutions/*.cpp"
)

# Automatically find all util source files
file(GLOB_RECURSE UTIL_SOURCES 
    "${CMAKE_CURRENT_SOURCE_DIR}/src/utils/*.cpp"
)

# Create executable
add_executable(aoc-rhyn0
    src/main.cpp
    ${SOLUTION_SOURCES}
    ${UTIL_SOURCES}
)

# Include directories
target_include_directories(aoc-rhyn0 PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

# Optional: Enable warnings
if(MSVC)
    target_compile_options(aoc-rhyn0 PRIVATE /W4 /WX)
else()
    target_compile_options(aoc-rhyn0 PRIVATE -Wall -Wextra -Werror)
endif()
