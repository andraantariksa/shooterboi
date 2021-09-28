#pragma once

#include <SDL.h>
#undef main

#include "Common.h"

class Game
{
public:
    static constexpr uint32_t window_width = 1280;
    static constexpr uint32_t window_height = 720;

    Game();
    ~Game();

    int run();

private:
    SDL_Window* m_window;
    SDL_GLContext m_ogl_context;
};
