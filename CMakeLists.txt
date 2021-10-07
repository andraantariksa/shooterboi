cmake_minimum_required(VERSION 3.8)

project(fps-raymarch)

add_subdirectory(extern/SDL)
add_subdirectory(extern/glad)
add_subdirectory(extern/reactphysics3d)
include(cmake/file_to_header.cmake)
include(cmake/ImGui.cmake)

add_executable(fps-raymarch
    src/Game.cpp
    src/Game.h
    src/Main.cpp  )

target_include_directories(fps-raymarch PUBLIC
    ${IMGUI_DIR}
    extern/glm)

target_link_libraries(fps-raymarch PUBLIC
    SDL2::SDL2-static
    glad
    IMGUI_LIBRARY)

convert_file_to_header_directory("src/shaders" "src/shaders/generated")

