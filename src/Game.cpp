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
#include "Camera.hpp"
#include "RenderObjects.hpp"

Game::Game() :
    m_window(nullptr)
{
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
        SDL_WINDOW_OPENGL);

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

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();

    ImGui_ImplSDL2_InitForOpenGL(m_window, m_ogl_context);
    ImGui_ImplOpenGL3_Init(glsl_version);

    m_engine.init();

    
    /*m_physics_world->setIsDebugRenderingEnabled(true);
    
    reactphysics3d::DebugRenderer& debugRenderer = m_physics_world->getDebugRenderer();

    debugRenderer.setIsDebugItemDisplayed(reactphysics3d::DebugRenderer::DebugItem::COLLISION_SHAPE, true);
    debugRenderer.setIsDebugItemDisplayed(reactphysics3d::DebugRenderer::DebugItem::CONTACT_POINT, true);
    debugRenderer.setIsDebugItemDisplayed(reactphysics3d::DebugRenderer::DebugItem::CONTACT_NORMAL, true);*/
}

Game::~Game()
{
    m_engine.shutdown();

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

    //GLuint fragment_shader_id = glCreateShader(GL_FRAGMENT_SHADER);
    //glShaderSource(fragment_shader_id, 1, &MAINPS, nullptr);
    //glCompileShader(fragment_shader_id);
    //
    //GLuint shader_program_handle = glCreateProgram();
    //glAttachShader(shader_program_handle, fragment_shader_id);
    //glLinkProgram(shader_program_handle);

    GLuint vertex_array_handle;
    glGenVertexArrays(1, &vertex_array_handle);
    glBindVertexArray(vertex_array_handle);

    GLuint vertex_buffer_handle;
    glGenBuffers(1, &vertex_buffer_handle);
    glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer_handle);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
    //glBindBuffer(GL_ARRAY_BUFFER, 0);

    /*
    GLuint ssbo_handle;
    glGenBuffers(1, &ssbo_handle);
    glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo_handle);
    glBufferData(GL_SHADER_STORAGE_BUFFER,
        m_render_objects.size() * sizeof(RenderObjectData),
        m_render_objects.data(),
        GL_DYNAMIC_COPY);
    glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, ssbo_handle);
    glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0);
    */
    
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(GLfloat), nullptr);
    glEnableVertexAttribArray(0);

    glBindVertexArray(vertex_array_handle);

    Camera camera(glm::vec3(8.0f, 3.0f, -8.0f));

    float system_time = 0.f;
    float delta_time = 0.f;
    auto start_time = std::chrono::high_resolution_clock::now();

    glm::vec2 reso = m_window_size;
    //init(m_registry, m_soloud, m_physics_world, m_physics_common, m_render_objects);

    bool running = true;
    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_WINDOWEVENT) {
                if (event.window.type == SDL_WINDOWEVENT_RESIZED) {
                    m_window_size.x = event.window.data1;
                    m_window_size.y = event.window.data2;

                    reso = m_window_size;
                }
            }
            /*
            else if (event.type == SDL_MOUSEMOTION) {
                float y_offset = event.motion.yrel;
                float x_offset = event.motion.xrel;

                camera.move_direction(glm::vec2(x_offset, y_offset));
            }*/
        }
        
        m_input_processor.process(event);

        /*
        std::cout
            << m_input_processor.get_mouse_acc().x
            << " , "
            << m_input_processor.get_mouse_acc().y
            << std::endl;
        */

        if (m_input_processor.is_action_key_down(ActionKey::ExitGame)) {
            running = false;
        }

        auto new_time = std::chrono::high_resolution_clock::now();
        float current_time = std::chrono::duration_cast<std::chrono::duration<float>>(new_time - start_time).count();
        delta_time = current_time - system_time;
        system_time = current_time;

        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplSDL2_NewFrame();
        ImGui::NewFrame();

        ImGui::Begin("Demo window", nullptr, ImGuiWindowFlags_AlwaysAutoResize);
        ImGui::Text("FPS: %f", 1.0f / delta_time);
        ImGui::End();

        m_engine.update(delta_time, m_input_processor, running, m_window);
        m_engine.render_scene(delta_time, reso);

        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        
        SDL_GL_SwapWindow(m_window);
    }

    //glDeleteBuffers(1, &ssbo_handle);
    glDeleteBuffers(1, &vertex_buffer_handle);
    glDeleteBuffers(1, &vertex_array_handle);

    return 0;
}

void Game::init()
{
    /*
    auto* sound_shoot = new SoLoud::Wav;
    sound_shoot->load("../../../assets/audio/shoot.wav");

    auto player_entity = registry.create();
    registry.emplace<Player>(player_entity);
    registry.emplace<Transform>(player_entity, glm::vec3(-5.0f, 10.0f, -5.0f));
    registry.emplace<AudioSourceShoot>(player_entity, soloud, std::unique_ptr<SoLoud::AudioSource>(sound_shoot));
    registry.emplace<AudioSourceShooted>(player_entity, soloud, std::unique_ptr<SoLoud::AudioSource>(sound_shoot));
    registry.emplace<RigidBody>(
        player_entity,
        physic_world,
        registry.get<Transform>(player_entity),
        reactphysics3d::BodyType::DYNAMIC,
        std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
        std::make_pair(physic_common.createBoxShape(reactphysics3d::Vector3(
            0.5f,
            0.5f,
            0.5f)),
            reactphysics3d::Transform()) }
    );

    auto base_terrain_entity = registry.create();
    registry.emplace<Transform>(base_terrain_entity, glm::vec3(0.0f));
    registry.emplace<RigidBody>(
        base_terrain_entity,
        physic_world,
        registry.get<Transform>(base_terrain_entity),
        reactphysics3d::BodyType::STATIC,
        std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
        std::make_pair(
            physic_common.createBoxShape(reactphysics3d::Vector3(
                100.0f,
                0.0001f,
                100.0f)),
            reactphysics3d::Transform()) }
    );

    auto enemy_entity = registry.create();
    registry.emplace<Transform>(enemy_entity, glm::vec3(0.0f, 5.0f, 0.0f));
    registry.emplace<RigidBody>(
        enemy_entity,
        physic_world,
        registry.get<Transform>(enemy_entity),
        reactphysics3d::BodyType::DYNAMIC,
        std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
        std::make_pair(
            physic_common.createSphereShape(0.5f),
            reactphysics3d::Transform()) }
    );
    render_objects.create(registry, enemy_entity, RenderObjectDataType::Enemy);*/
}
