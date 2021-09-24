#include "Game.h"
#include <glad/glad.h>

Game::Game() :
    window_(nullptr)
{
    if (SDL_Init(SDL_INIT_EVERYTHING) < 0) {
        assert(false && "Cannot initialize SDL");
    }

    window_ = SDL_CreateWindow(
        "FPS",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        window_width,
        window_height,
        SDL_WINDOW_OPENGL);

    if (window_ == nullptr) {
        assert(false && "Cannot create window");
    }

    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GL_RED_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_ALPHA_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);

    ogl_context_ = SDL_GL_CreateContext(window_);

    if (ogl_context_ == nullptr) {
        assert(false && "Cannot create OpenGL context");
    }

    SDL_GL_MakeCurrent(window_, ogl_context_);

    if (!gladLoadGL()) {
        assert(false && "Cannot create load OpenGL");
    }

    SDL_GL_SetSwapInterval(1);
}

Game::~Game()
{
    SDL_GL_DeleteContext(window_);
    SDL_DestroyWindow(window_);
    SDL_Quit();
}

int Game::run()
{
    bool running = true;

    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
        }

        glClearColor(1.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        SDL_GL_SwapWindow(window_);
    }

    return 0;
}
