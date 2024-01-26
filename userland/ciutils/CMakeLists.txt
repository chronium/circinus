cmake_minimum_required(VERSION 3.20)

set(CMAKE_TOOLCHAIN_FILE /home/chronium/repos/circinus/toolchain/circinus_toolchain.cmake)

project(ciutils C)

add_executable(hw hw/hw.main.c)
add_executable(echo echo/echo.main.c)
install(TARGETS hw echo DESTINATION /home/chronium/repos/circinus/userland/build)
#add_executable(cat cat/cat.main.c)
#add_executable(ls ls/ls.main.c)
#add_executable(echo echo/echo.main.c)
