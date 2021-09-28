cmake_minimum_required(VERSION 3.8)

project(fps-raymarch)

add_subdirectory(extern/SDL)
add_subdirectory(extern/glad)

add_executable(fps-raymarch
    src/Game.cpp
    src/Game.h
    src/Main.cpp)

target_link_libraries(fps-raymarch PUBLIC
    SDL2::SDL2-static
    glad)
