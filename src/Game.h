#pragma once

#include <SDL.h>
#undef main
#include <glm/glm.hpp>

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
};
