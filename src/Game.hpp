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
#include "Engine.hpp"

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
    Engine m_engine;

    void init();
};
