#pragma once

#include <SDL.h>
#undef main
#include <glm/glm.hpp>
#include <entt/entt.hpp>
#include <reactphysics3d/reactphysics3d.h>
#include <soloud.h>

#include "Common.h"
#include "InputProcessor.h"

class Game
{
public:
    Game();
    ~Game();

    int run();

private:
    glm::ivec2 m_window_size{ 1280, 720 };
    SDL_Window* m_window;
    SDL_GLContext m_ogl_context;

    InputProcessor m_input_processor;

    reactphysics3d::PhysicsCommon m_physics_common;
    reactphysics3d::PhysicsWorld* m_physics_world;

    SoLoud::Soloud m_soloud;
    entt::registry m_registry;
    float m_yaw = 0.0f;
    float m_pitch = 0.0f;
};
