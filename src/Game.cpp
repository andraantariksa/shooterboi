#include "Game.h"
#include <array>
#include <iostream>
#include <chrono>
#include <glad/glad.h>
#include <SDL.h>
#include <SDL_opengl.h>
#include <imgui.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <backends/imgui_impl_sdl.h>
#include <backends/imgui_impl_opengl3.h>

#include "shaders/generated/main_PS.hpp"

struct GameObject {
};

Game::Game() :
    m_window(nullptr)
{
    if (SDL_Init(SDL_INIT_EVERYTHING) < 0)
    {
        assert(false && "Cannot initialize SDL");
    }

    m_window = SDL_CreateWindow(
        "FPS",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        static_cast<uint32_t>(m_window_size.x),
        static_cast<uint32_t>(m_window_size.y),
        SDL_WINDOW_OPENGL);

    if (m_window == nullptr)
    {
        assert(false && "Cannot create window");
    }

    const char* glsl_version = "#version 430";

    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 0);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GL_RED_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_ALPHA_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);

    m_ogl_context = SDL_GL_CreateContext(m_window);

    if (m_ogl_context == nullptr)
    {
        assert(false && "Cannot create OpenGL context");
    }

    SDL_GL_MakeCurrent(m_window, m_ogl_context);

    if (!gladLoadGL())
    {
        assert(false && "Cannot create load OpenGL");
    }

    SDL_GL_SetSwapInterval(1);

    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();
    (void)io;

    ImGui_ImplSDL2_InitForOpenGL(m_window, m_ogl_context);
    ImGui_ImplOpenGL3_Init(glsl_version);
}

Game::~Game()
{
    SDL_GL_DeleteContext(m_window);
    SDL_DestroyWindow(m_window);
    SDL_Quit();
}

int Game::run()
{
    GLfloat vertices[] = {
        -1.f, -1.f, 0.f,
        -1.f, 1.f, 0.f,
        1.f, -1.f, 0.f,
        1.f, 1.f, 0.f,
    };

    std::array<GameObject, 1000> gameobjects_buffer;

    GLuint fragment_shader_id = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragment_shader_id, 1, &MAIN_PS, nullptr);
    glCompileShader(fragment_shader_id);

    GLuint shader_program_handle = glCreateProgram();
    glAttachShader(shader_program_handle, fragment_shader_id);
    glLinkProgram(shader_program_handle);

    GLuint vertex_array_handle;
    glGenVertexArrays(1, &vertex_array_handle);
    glBindVertexArray(vertex_array_handle);

    GLuint vertex_buffer_handle;
    glGenBuffers(1, &vertex_buffer_handle);
    glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer_handle);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    GLuint ssbo_handle;
    glGenBuffers(1, &ssbo_handle);
    glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo_handle);
    glBufferData(GL_SHADER_STORAGE_BUFFER, gameobjects_buffer.size() * sizeof(GameObject), gameobjects_buffer.data(), GL_DYNAMIC_COPY);
    
    
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(GLfloat), nullptr);
    glEnableVertexAttribArray(0);

    glBindVertexArray(vertex_array_handle);

    // Uniform
    /*unsigned int uniform_buffer_handle;
    glGenBuffers(1, &uniform_buffer_handle);
    glBindBuffer(GL_UNIFORM_BUFFER, uniform_buffer_handle);
    glBufferData(GL_UNIFORM_BUFFER, 152, NULL, GL_STATIC_DRAW);
    glBindBuffer(GL_UNIFORM_BUFFER, 0);

    GLuint uniform_indices_uCameraPosition = glGetUniformBlockIndex(shader_program_handle, "uCameraPosition");
    GLuint uniform_indices_uResolution = glGetUniformBlockIndex(shader_program_handle, "uResolution");
    GLuint uniform_indices_uTime = glGetUniformBlockIndex(shader_program_handle, "uTime");*/
    // End uniform

    SDL_SetRelativeMouseMode(SDL_TRUE);

    auto camera_position = glm::vec3(8.0f, 3.0f, -8.0f);
    auto camera_direction = glm::vec3(0.0f, 0.0f, -1.0f);

    float system_time = 0.f;
    float delta_time = 0.f;
    auto start_time = std::chrono::system_clock::now();

    GLint uniform_location_uTime = glGetUniformLocation(shader_program_handle, "uTime");
    GLint uniform_location_uResolution = glGetUniformLocation(shader_program_handle, "uResolution");
    GLint uniform_location_uCameraPosition = glGetUniformLocation(shader_program_handle, "uCameraPosition");
    GLint uniform_location_uCameraDirection = glGetUniformLocation(shader_program_handle, "uCameraDirection");

    bool running = true;
    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT || (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE)) {
                running = false;
            }
            else if (event.type == SDL_WINDOWEVENT)
            {
                if (event.window.type == SDL_WINDOWEVENT_RESIZED)
                {
                    m_window_size.x = event.window.data1;
                    m_window_size.y = event.window.data2;
                }
            }
            else if (event.type == SDL_MOUSEMOTION)
            {
                std::cout << camera_direction.x << camera_direction.y << camera_direction.z << '\n';
                glm::vec4 camera_direction_(camera_direction, 1.0f);
                camera_direction_ =
                    glm::rotate(
                        glm::mat4(1.0f),
                        glm::radians(static_cast<float>(event.motion.xrel)),
                        glm::vec3(0.0f, 1.0f, 0.0f)) *
                    glm::rotate(
                        glm::mat4(1.0f),
                        glm::radians(static_cast<float>(event.motion.yrel)),
                        glm::vec3(1.0f, 0.0f, 0.0f)) *
                    camera_direction_;
                camera_direction_ /= camera_direction_.w;
                camera_direction = camera_direction_;
            }


            if (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_w)
            {
                camera_position += camera_direction * 3.0f * delta_time;
            }
            else if (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_s)
            {
                camera_position -= camera_direction * 3.0f * delta_time;
            }

            if (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_d)
            {
                camera_position += glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction) * 3.0f * delta_time;
            }
            else if (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_a)
            {
                camera_position -= glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction) * 3.0f * delta_time;
            }
        }

        /*camera_direction = glm::rotate(
            glm::mat4(1.0f),
            glm::radians(1.0f),
            glm::vec3(0.0f, 1.0f, 0.0f)) *
            glm::vec4(camera_direction, 1.0f);*/

        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        float current_time = std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::system_clock::now() - start_time).count() / 1000000.0f;
        delta_time = current_time - system_time;
        system_time = current_time;

        glUseProgram(shader_program_handle);
        glUniform2fv(uniform_location_uResolution, 1, glm::value_ptr(m_window_size));
        glUniform3fv(uniform_location_uCameraPosition, 1, glm::value_ptr(camera_position));
        glUniform3fv(uniform_location_uCameraDirection, 1, glm::value_ptr(camera_direction));
        glUniform1f(uniform_location_uTime, current_time);
        glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

        SDL_GL_SwapWindow(m_window);
    }

    glDeleteProgram(shader_program_handle);
    glDeleteShader(fragment_shader_id);

    return 0;
}
