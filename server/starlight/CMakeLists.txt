cmake_minimum_required(VERSION 3.20)

set(CMAKE_TOOLCHAIN_FILE /home/chronium/repos/circinus/toolchain/circinus_toolchain.cmake)

project(sl C)

file(GLOB_RECURSE SOURCES src/*.c)

add_executable(sl ${SOURCES})
target_include_directories(sl PRIVATE include)

install(TARGETS sl DESTINATION /home/chronium/repos/circinus/userland/build)