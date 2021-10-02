#pragma once

#include <SDL.h>
#undef main
#include <glm/glm.hpp>

#include "Common.h"

class Game
{
public:
    Game();
    ~Game();

    int run();

private:
    glm::vec2 m_window_size = glm::vec2(1280.0f, 720.f);
    SDL_Window* m_window;
    SDL_GLContext m_ogl_context;
};
