cmake_minimum_required(VERSION 3.8)

project(fps-raymarch)

set(CMAKE_CXX_STANDARD 17)

add_subdirectory(extern/SDL)

set(SDL2_LIBRARY SDL2::SDL2-static)
set(SDL2_INCLUDE_DIR "extern/SDL/include")

add_subdirectory(extern/glad)
add_subdirectory(extern/reactphysics3d)
add_subdirectory(extern/soloud/contrib)
include(cmake/file_to_header.cmake)
include(cmake/ImGui.cmake)

add_executable(fps-raymarch
    "src/Game.cpp"
    "src/Game.hpp"
    "src/InputProcessor.cpp"
    "src/InputProcessor.hpp"
    "src/Main.cpp"
    "src/logic/systems/Systems.hpp"
    "src/logic/systems/Systems.cpp"
    "src/logic/components/Player.hpp"
    "src/Camera.hpp"
    "src/Common.hpp"
    "src/logic/components/RigidBody.hpp"
    "src/logic/components/AudioSource.hpp"
    "src/logic/components/AudioSource3D.hpp"
    "src/RenderObjectData.hpp"
    "src/RenderObjects.hpp"
    "src/logic/components/RenderObject.hpp" "src/logic/custom-components/player/AudioSourceShooted.hpp" "src/utils/Converter.hpp")

target_include_directories(fps-raymarch PUBLIC
    "extern/glm"
    "extern/entt/single_include"
    "extern/reactphysics3d/include"
    "extern/soloud/include")

target_link_libraries(fps-raymarch PUBLIC
    SDL2::SDL2-static
    glad
    imgui
    reactphysics3d
    soloud)

convert_file_to_header_directory("src/shaders" "src/shaders/generated")

