#pragma once

#include <SDL.h>
#undef main
#include <glm/glm.hpp>
#include <entt/entt.hpp>
#include <reactphysics3d/reactphysics3d.h>
#include <soloud.h>

#include "Common.hpp"
#include "InputProcessor.hpp"
#include "RenderObjects.hpp"

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

    RenderObjects<100> m_render_objects;

    SoLoud::Soloud m_soloud;
    entt::registry m_registry;
};
