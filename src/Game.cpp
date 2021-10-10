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
#include <reactphysics3d/reactphysics3d.h>
#include <soloud.h>
#include <soloud_wav.h>
#define STB_IMAGE_IMPLEMENTATION
#include <stb_image.h>

#include "RenderObjectData.hpp"
#include "logic/systems/Systems.hpp"
#include "Game.hpp"
#include "shaders/generated/mainPS.hpp"
#include "Camera.hpp"
#include "RenderObjects.hpp"

Game::Game() :
    m_window(nullptr) {
    if (SDL_Init(SDL_INIT_EVERYTHING) < 0) {
        std::cout << SDL_GetError() << '\n';
        assert(false && "Cannot initialize SDL");
    }

    m_window = SDL_CreateWindow(
        "FPS",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        m_window_size.x,
        m_window_size.y,
        SDL_WINDOW_OPENGL |
        SDL_WINDOW_INPUT_GRABBED);

    if (m_window == nullptr) {
        assert(false && "Cannot create window");
    }

    m_input_processor.init();

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

    m_physics_world = m_physics_common.createPhysicsWorld();
    m_physics_world->setIsGravityEnabled(true);
    m_physics_world->setGravity(reactphysics3d::Vector3(0, -9.81, 0));
    /*m_physics_world->setIsDebugRenderingEnabled(true);
    
    reactphysics3d::DebugRenderer& debugRenderer = m_physics_world->getDebugRenderer();

    debugRenderer.setIsDebugItemDisplayed(reactphysics3d::DebugRenderer::DebugItem::COLLISION_SHAPE, true);
    debugRenderer.setIsDebugItemDisplayed(reactphysics3d::DebugRenderer::DebugItem::CONTACT_POINT, true);
    debugRenderer.setIsDebugItemDisplayed(reactphysics3d::DebugRenderer::DebugItem::CONTACT_NORMAL, true);*/

    m_soloud.init();
}

Game::~Game()
{
    m_registry.each([&](auto& entity) {
        m_registry.remove_all(entity);
        m_registry.destroy(entity);
    });

    m_soloud.deinit();

    m_physics_common.destroyPhysicsWorld(m_physics_world);

    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplSDL2_Shutdown();
    ImGui::DestroyContext();

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

    //std::array<RenderObjectData, 100> renderobjectdata_buffer;
    //renderobjectdata_buffer[0].m_type = RenderObjectDataType::Enemy;
    //renderobjectdata_buffer[0].m_data.enemies.enemies.m_position = glm::vec3(-2.0f, 0.5f, -2.0f);
    //renderobjectdata_buffer[1].m_type = RenderObjectDataType::Enemy;
    //renderobjectdata_buffer[1].m_data.enemies.enemies.m_position = glm::vec3(-4.0f, 1.5f, -4.0f);

    int width, height, nrChannels;
    GLuint textures[1];
    uint8_t* texture_data = stbi_load("../../../assets/texture/ground.jpg", &width, &height, &nrChannels, 0);
    if (texture_data) {
        glGenTextures(1, textures);
        glBindTexture(GL_TEXTURE_2D, textures[0]);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, texture_data);

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
        glGenerateMipmap(GL_TEXTURE_2D);
    }
    stbi_image_free(texture_data);

    GLuint fragment_shader_id = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragment_shader_id, 1, &MAINPS, nullptr);
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
    //glBindBuffer(GL_ARRAY_BUFFER, 0);

    GLuint ssbo_handle;
    glGenBuffers(1, &ssbo_handle);
    glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo_handle);
    glBufferData(GL_SHADER_STORAGE_BUFFER,
        m_render_objects.size() * sizeof(RenderObjectData),
        m_render_objects.data(),
        GL_DYNAMIC_COPY);
    glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, ssbo_handle);
    glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0);
    
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(GLfloat), nullptr);
    glEnableVertexAttribArray(0);

    glBindVertexArray(vertex_array_handle);

    SDL_SetRelativeMouseMode(SDL_TRUE);

    Camera camera(glm::vec3(8.0f, 3.0f, -8.0f));

    float system_time = 0.f;
    float delta_time = 0.f;
    auto start_time = std::chrono::high_resolution_clock::now();

    GLint uniform_location_uTextureGround = glGetUniformLocation(shader_program_handle, "uTextureGround");
    GLint uniform_location_uTime = glGetUniformLocation(shader_program_handle, "uTime");
    GLint uniform_location_uResolution = glGetUniformLocation(shader_program_handle, "uResolution");
    GLint uniform_location_uCameraPosition = glGetUniformLocation(shader_program_handle, "uCameraPosition");
    GLint uniform_location_uCameraDirection = glGetUniformLocation(shader_program_handle, "uCameraDirection");

    glm::vec2 reso = m_window_size;
    init(m_registry, m_soloud, m_physics_world, m_physics_common, m_render_objects);

    bool running = true;
    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            m_input_processor.process(event);
            if (event.type == SDL_QUIT || (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE)) {
                running = false;
            }
            else if (event.type == SDL_WINDOWEVENT)
            {
                if (event.window.type == SDL_WINDOWEVENT_RESIZED)
                {
                    m_window_size.x = event.window.data1;
                    m_window_size.y = event.window.data2;

                    reso = m_window_size;
                }
            }
            else if (event.type == SDL_MOUSEMOTION) {
                float y_offset = event.motion.yrel;
                float x_offset = event.motion.xrel;

                camera.move_direction(glm::vec2(x_offset, y_offset));
            }
        }

        if (m_input_processor.is_action_key_down(ActionKey::ExitGame)) {
            running = false;
        }
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        auto new_time = std::chrono::high_resolution_clock::now();
        float current_time = std::chrono::duration_cast<std::chrono::duration<float>>(new_time - start_time).count();
        delta_time = current_time - system_time;
        system_time = current_time;

        // Update
        update(m_registry, delta_time, m_input_processor, camera, m_soloud, m_physics_world, m_physics_common, m_render_objects);
        m_soloud.update3dAudio();
        m_physics_world->update(delta_time);

        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplSDL2_NewFrame();
        ImGui::NewFrame();

        ImGui::Begin("Demo window");
        ImGui::Text("FPS: %f", 1.0f / delta_time);
        ImGui::End();

        // Copy to SSBO
        glBindBuffer(GL_UNIFORM_BUFFER, ssbo_handle);
        void* buff_ptr = glMapBuffer(GL_UNIFORM_BUFFER, GL_WRITE_ONLY);
        std::memcpy(buff_ptr, m_render_objects.data(), m_render_objects.size());
        glUnmapBuffer(GL_UNIFORM_BUFFER);

        // Render
        ImGui::Render();
        glViewport(0, 0, m_window_size.x, m_window_size.y);
        glUseProgram(shader_program_handle);
        glUniform1i(uniform_location_uTextureGround, 0);
        glUniform2fv(uniform_location_uResolution, 1, glm::value_ptr(reso));
        glUniform3fv(uniform_location_uCameraPosition, 1, glm::value_ptr(camera.m_position));
        glUniform3fv(uniform_location_uCameraDirection, 1, glm::value_ptr(camera.get_direction()));
        glUniform1f(uniform_location_uTime, current_time);

        glActiveTexture(GL_TEXTURE0 + 0);
        glBindTexture(GL_TEXTURE_2D, textures[0]);

        glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());

        SDL_GL_SwapWindow(m_window);
    }

    glDeleteProgram(shader_program_handle);
    glDeleteShader(fragment_shader_id);

    return 0;
}
