#pragma once

#include <reactphysics3d/reactphysics3d.h>
#include <entt/entt.hpp>
#include <soloud.h>
#include <soloud_wav.h>
#include <vector>

#include "RenderObjects.hpp"
#include "InputProcessor.hpp"
#include "Renderer.hpp"

using AudioResourceID = uint32_t;

class Engine
{
public:
    Engine();
    ~Engine();

    void init();
    AudioResourceID load_audio_resource(const char* path);
    void update(float dt, const InputProcessor& input_processor);
    void render_scene(const glm::vec2& resolution);
    void shutdown();

private:
    reactphysics3d::PhysicsCommon m_physics_common;
    reactphysics3d::PhysicsWorld* m_physics_world;
    entt::registry m_registry;
    entt::entity m_player_entity{};
    entt::entity m_terrain_entity{};
    
    SoLoud::Soloud m_soloud;
    std::vector<SoLoud::Wav*> m_audio_resources;
    AudioResourceID m_audio_res_id_counter;

    Renderer m_renderer;

    RenderObjects<100> m_render_objects{};
};
