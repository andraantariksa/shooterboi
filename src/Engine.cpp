#include "Engine.hpp"
#include "Camera.hpp"
#include "InputProcessor.hpp"
#include "logic/components/Transform.hpp"
#include "logic/components/Player.hpp"
#include "logic/components/Collider.hpp"
#include "logic/components/RigidBody.hpp"
#include "logic/components/FrustumCullObject.hpp"
#include "logic/components/Enemy.hpp"
#include "logic/custom-components/player/AudioSourceShoot.hpp"
#include "logic/custom-components/player/AudioSourceShooted.hpp"
#include "utils/Converter.hpp"
#include "logic/components/GameMusic.hpp"
#include "logic/components/SelfDestruct.hpp"
#include "ShootRaycast.hpp"

#include <imgui.h>
#include <random>
#include "Game.hpp"
#include <limits>
#include "logic/components/FrustumCullObject.hpp"

Engine::Engine() :
    m_physics_world(nullptr),
    m_audio_res_id_counter(0)
{
}

Engine::~Engine()
{
}

void Engine::init()
{
    m_renderer.init();
    m_soloud.init();

    m_physics_world = m_physics_common.createPhysicsWorld();
    m_physics_world->setIsGravityEnabled(true);
    m_physics_world->setGravity(reactphysics3d::Vector3(0.0f, -9.81f, 0.0f));

    generate_map();

    //auto* sound_shoot = new SoLoud::Wav;
    //sound_shoot->load("../../../assets/audio/shoot.wav");

    auto game_manager_entity = m_registry.create();
    m_registry.emplace<GameMusic>(
        game_manager_entity,
        *this,
        load_audio_resource("../../../assets/audio/deadship.ogg"),
        load_audio_resource("../../../assets/audio/deadship.ogg"));

    {
        m_player_entity = m_registry.create();
        m_registry.emplace<Player>(m_player_entity);
        m_registry.emplace<Transform>(m_player_entity, glm::vec3(0.0f, 3.0f, -3.0f));
        //m_registry.emplace<AudioSourceShoot>(m_player_entity, m_soloud, std::unique_ptr<SoLoud::AudioSource>(sound_shoot));
        //m_registry.emplace<AudioSourceShooted>(m_player_entity, m_soloud, std::unique_ptr<SoLoud::AudioSource>(sound_shoot));
        m_registry.emplace<RigidBody>(
            m_player_entity,
            m_physics_world,
            m_registry.get<Transform>(m_player_entity),
            reactphysics3d::BodyType::DYNAMIC,
            std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
            std::make_pair(m_physics_common.createBoxShape(reactphysics3d::Vector3(
                0.5f,
                0.5f,
                0.5f)),
                reactphysics3d::Transform()) }
        );
        m_registry.emplace<AudioSourceShoot>(m_player_entity, *this, load_audio_resource("../../../assets/audio/shoot.wav"));

        auto& player_rigidbody = m_registry.get<RigidBody>(m_player_entity);
        player_rigidbody.getCollider(0)->getMaterial().setBounciness(0.0f);
    }

    auto ground_entity = m_registry.create();
    m_registry.emplace<Transform>(ground_entity, glm::vec3(0.0f));
    m_registry.emplace<RigidBody>(
        ground_entity,
        m_physics_world,
        m_registry.get<Transform>(ground_entity),
        reactphysics3d::BodyType::STATIC,
        std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
        std::make_pair(m_physics_common.createBoxShape(reactphysics3d::Vector3(
            100.0f,
            0.0001f,
            100.0f)),
            reactphysics3d::Transform()) }
    );

    {
        auto enemy_entity = m_registry.create();
        m_registry.emplace<Enemy>(enemy_entity);
        m_registry.emplace<Transform>(enemy_entity, glm::vec3(0.0f, 3.0f, 0.0f));
        m_registry.emplace<Renderable>(
            enemy_entity,
            RenderObjectType::Object,
            ShapeType::Sphere,
            glm::vec3(glm::vec3(46.f, 209.f, 162.f) / 255.f));
        m_registry.emplace<RigidBody>(
            enemy_entity,
            m_physics_world,
            m_registry.get<Transform>(enemy_entity),
            reactphysics3d::BodyType::DYNAMIC,
            std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
            std::make_pair(m_physics_common.createBoxShape(reactphysics3d::Vector3(
                0.5f,
                0.5f,
                0.5f)),
                reactphysics3d::Transform()) }
        );

        auto& rigid_body = m_registry.get<RigidBody>(enemy_entity);
        rigid_body.getRigidBody()->setUserData(new RigidBodyData{ RigidBodyType::Enemy, enemy_entity });

        auto& enemy_renderable = m_registry.get<Renderable>(enemy_entity);
        enemy_renderable.shape_sphere.radius = 0.5f;
    }


    std::mt19937 gen;
    std::uniform_real_distribution<float> dis(1.0f, 5.0f);
    for (uint32_t i = 0; i < 10; i++) {
        auto test_sphere = m_registry.create();
        m_registry.emplace<Transform>(test_sphere, glm::vec3(dis(gen), 2.0f, dis(gen)));
        m_registry.emplace<Renderable>(
            test_sphere,
            RenderObjectType::Object,
            ShapeType::Sphere,
            glm::vec3(glm::vec3(46.f, 209.f, 162.f) / 255.f));
        m_registry.emplace<RigidBody>(
            test_sphere,
            m_physics_world,
            m_registry.get<Transform>(test_sphere),
            reactphysics3d::BodyType::DYNAMIC,
            std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
            std::make_pair(m_physics_common.createSphereShape(0.5f),
                reactphysics3d::Transform()) }
        );
        m_registry.emplace<FrustumCullObject>(test_sphere, 0.5f);

        // set shape data
        auto& test_sphere_renderable = m_registry.get<Renderable>(test_sphere);
        test_sphere_renderable.shape_sphere.radius = 0.5f;
    }
    
    /*
    auto enemy_entity = m_registry.create();
    m_registry.emplace<Transform>(enemy_entity, glm::vec3(0.0f, 5.0f, 0.0f));
    m_registry.emplace<RigidBody>(
        enemy_entity,
        m_physics_world,
        m_registry.get<Transform>(enemy_entity),
        reactphysics3d::BodyType::DYNAMIC,
        std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
        std::make_pair(
            m_physics_common.createSphereShape(0.5f),
            reactphysics3d::Transform()) }
    );*/

    auto& imgui_io = ImGui::GetIO();
    imgui_io.FontGlobalScale = 2.0f;
    auto& imgui_style = ImGui::GetStyle();
    imgui_style.ScaleAllSizes(2.0f);
}

AudioResourceID Engine::load_audio_resource(const char* path)
{
    auto res = new SoLoud::Wav();
    res->load(path);
    m_audio_resources.push_back(res);
    return m_audio_res_id_counter++;
}

void Engine::update(float delta_time, const InputProcessor& input_processor, bool& running, SDL_Window* window)
{
    ImGuiIO& imgui_io = ImGui::GetIO();
    switch (m_game_state) {
    case GameState::Game:
    {
        SDL_SetRelativeMouseMode(SDL_TRUE);
        //auto camera_direction = camera.get_direction();
    //auto camera_direction_horizontal = camera.get_direction_without_pitch();

        {
            auto collider_view = m_registry.view<RigidBody, Transform>();
            for (const auto& entity : collider_view) {
                auto& transform = collider_view.get<Transform>(entity);
                auto& collider = collider_view.get<RigidBody>(entity);

                const auto& collider_transform = collider.getTransform();
                const auto& collider_position = collider_transform.getPosition();
                transform.m_position = glm::vec3(collider_position.x, collider_position.y, collider_position.z);
            }
        }

        // update player movement
        {
            auto& transform = m_registry.get<Transform>(m_player_entity);
            auto& rigid_body = m_registry.get<RigidBody>(m_player_entity);
            auto& player = m_registry.get<Player>(m_player_entity);
            auto& audio_shoot = m_registry.get<AudioSourceShoot>(m_player_entity);

            if (input_processor.is_mouse_moving()) {
                player.move_direction(input_processor.get_mouse_acc());
            }

            auto camera_direction_horizontal = player.get_direction();

            if (input_processor.is_action_key_down(ActionKey::MoveForward)) {
                transform.m_position += camera_direction_horizontal * 3.0f * delta_time;
            }
            else if (input_processor.is_action_key_down(ActionKey::MoveBackward)) {
                transform.m_position -= camera_direction_horizontal * 3.0f * delta_time;
            }

            if (input_processor.is_action_key_down(ActionKey::MoveRight)) {
                transform.m_position += glm::normalize(glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal)) * 3.0f * delta_time;
            }
            else if (input_processor.is_action_key_down(ActionKey::MoveLeft)) {
                transform.m_position -= glm::normalize(glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal)) * 3.0f * delta_time;
            }

            if (input_processor.is_action_key_down(ActionKey::Jump)) {
                rigid_body.getRigidBody()->setLinearVelocity(reactphysics3d::Vector3(0.0f, 5.0f, 0.0f));
            }

            if (input_processor.is_mouse_pressed(MouseButton::Left) && player.is_ready_to_shoot_and_refresh()) {
                audio_shoot.play(*this);

                auto ray_from = transform.m_position + player.get_direction() * 0.0f + player.get_direction_right() * 0.008f;
                auto ray_to = ray_from + player.get_direction() * 999.0f;

                auto laser_ray_entity = m_registry.create();
                m_registry.emplace<Transform>(laser_ray_entity);
                auto& laser_ray_renderable = m_registry.emplace<Renderable>(
                    laser_ray_entity,
                    RenderObjectType::Object,
                    ShapeType::CapsuleLine,
                    glm::vec3(glm::vec3(46.f, 209.f, 162.f) / 255.f));
                laser_ray_renderable.shape_capsule_line.from = ray_from;
                laser_ray_renderable.shape_capsule_line.to = ray_to;
                laser_ray_renderable.shape_capsule_line.radius = 0.001f;
                m_registry.emplace<SelfDestruct>(laser_ray_entity, 0.05f);

                ShootRaycast shoot_raycast(&m_registry, m_physics_world);

                reactphysics3d::Ray shoot_ray(to_react(ray_from), to_react(ray_to));
                m_physics_world->raycast(shoot_ray, &shoot_raycast);
            }
        }

        {
            auto& enemies_view = m_registry.view<Enemy, Transform>();
            for (const auto& entity : enemies_view) {
                auto& enemy = enemies_view.get<Enemy>(entity);
                auto& player_transform = m_registry.get<Transform>(m_player_entity);
                auto& this_transform = enemies_view.get<Transform>(entity);
                if (true)
                {
                    this_transform.m_position += glm::normalize(player_transform.m_position - this_transform.m_position) * enemy.speed * delta_time;
                }
            }
        }

        {
            auto collider_view = m_registry.view<SelfDestruct>();
            for (const auto& entity : collider_view) {
                auto& transform = collider_view.get<SelfDestruct>(entity);
                if (transform.update_and_is_ready_to_delete()) {
                    m_registry.destroy(entity);
                }
            }
        }

        {
            auto collider_view = m_registry.view<RigidBody, Transform>();
            for (const auto& entity : collider_view) {
                auto& transform = collider_view.get<Transform>(entity);
                auto& collider = collider_view.get<RigidBody>(entity);

                collider.setTransform(transform);
            }
        }

        m_physics_world->update(delta_time);
        m_soloud.update3dAudio();

        ImGui::SetNextWindowPos(ImVec2(imgui_io.DisplaySize.x / 2.0f, imgui_io.DisplaySize.y / 2.0f), 0, ImVec2(0.5f, 0.5f));
        ImGui::Begin("#Pause Menu", nullptr, ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_AlwaysAutoResize);
        ImGui::Button("Resume");
        ImGui::Button("Settings");
        ImGui::Button("Quit");
        ImGui::End();

        ImGui::SetNextWindowPos(ImVec2(0.0f, 0.0f), 0, ImVec2(0.0f, 0.0f));
        ImGui::Begin("#Quest", nullptr, ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_AlwaysAutoResize);
        ImGui::Text("Quest");
        ImGui::Text("8/8");
        ImGui::End();

        ImGui::SetNextWindowPos(ImVec2(imgui_io.DisplaySize.x, imgui_io.DisplaySize.y), 0, ImVec2(1.0f, 1.0f));
        ImGui::Begin("#Health", nullptr, ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_AlwaysAutoResize);
        float health = 100.0f;
        ImGui::DragFloat("Health", &health);
        ImGui::End();
    }
    break;
    case GameState::MainMenu:
    {
        static bool open_window_settings = false;

        ImGui::SetNextWindowPos(ImVec2(0.0f, imgui_io.DisplaySize.y), 0, ImVec2(0.0f, 1.0f));
        {
            ImGui::Begin("#Main Menu", nullptr, ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_AlwaysAutoResize);
            if (ImGui::Button("Start")) {
                m_game_state = GameState::Game;
            }
            if (ImGui::Button("Settings")) {
                open_window_settings = true;
            }
            ImGui::Button("Guide");
            if (ImGui::Button("Quit")) {
                running = false;
            }
        }
        ImGui::End();

        if (open_window_settings) {
            ImGui::SetNextWindowPos(ImVec2(imgui_io.DisplaySize.x / 2.0f, imgui_io.DisplaySize.y / 2.0f), 0, ImVec2(0.5f, 0.5f));
            ImGui::Begin("Settings", &open_window_settings, ImGuiWindowFlags_AlwaysAutoResize);
            int v = 0;
            ImGui::SliderInt("March maximum step", &v, 100, 1000);
            ImGui::SliderInt("Ambient occlusion sample", &v, 1, 50);
            ImGui::End();
        }
    }
    break;
    }
}

void Engine::render_scene(float delta_time, const glm::vec2& resolution)
{
    if (m_game_state == GameState::Game) {
        m_renderer.begin();

        auto renderable_view = m_registry.view<Renderable, Transform>(entt::exclude<FrustumCullObject>);
        for (const auto& entity : renderable_view) {
            m_renderer.submit(
                renderable_view.get<Transform>(entity),
                renderable_view.get<Renderable>(entity));
        }

        const auto& player_transform = m_registry.get<Transform>(m_player_entity);
        auto& player = m_registry.get<Player>(m_player_entity);
        auto frustum = player.get_frustum(player_transform);

        auto renderable_view_with_frustum = m_registry.view<Renderable, Transform, FrustumCullObject>();
        int i = 0;
        for (const auto& entity : renderable_view_with_frustum) {
            auto& frustum_cull_object = renderable_view_with_frustum.get<FrustumCullObject>(entity);
            auto& transform = renderable_view_with_frustum.get<Transform>(entity);
            if (frustum_cull_object.is_inside_frustum(transform.m_position, frustum)) {
                m_renderer.submit(
                    transform,
                    renderable_view_with_frustum.get<Renderable>(entity));
                i++;
            }
        }
        std::cout << i << '\n';

        m_renderer.end();
    }

    auto& player_transform = m_registry.get<Transform>(m_player_entity);
    auto& player = m_registry.get<Player>(m_player_entity);
    auto player_dir = player.get_direction();

    m_renderer.render(
        delta_time,
        resolution,
        player_transform.m_position,
        player_dir,
        m_game_state == GameState::Game);
}

void Engine::shutdown()
{
    m_registry.each([&](auto& entity) {
        m_registry.destroy(entity);
    });

    m_physics_common.destroyPhysicsWorld(m_physics_world);
    m_soloud.deinit();
    m_renderer.shutdown();
}

void Engine::generate_map()
{
    char* map_data = new char[64 * 64];
    std::mt19937 gen;
    std::uniform_real_distribution<float> dist(0.f, 1.f);
    auto random = [](const glm::vec2& seed) {
        return glm::fract(glm::sin(glm::dot(seed, glm::vec2(12.9898, 78.233))) * 43758.5453);
    };

    std::cout << "Generating level..." << std::endl;

    for (uint32_t y = 0; y < 64; y++) {
        for (uint32_t x = 0; x < 64; x++) {
            glm::vec2 pos = glm::vec2((float)x, (float)y) / 64.f;
            glm::vec2 tile_id = floor(pos);
            glm::vec2 tile_pos = glm::smoothstep(0.0f, 1.0f, fract(pos)); // make the interpolation smoother

            // get random values at four corners
            // c0 ------- c1
            //  |		  |
            //  |   	  |
            //  |		  |
            // c2 ------- c3
            float c0 = random(tile_id);
            float c1 = random(tile_id + glm::vec2(1.0, 0.0));
            float c2 = random(tile_id + glm::vec2(0.0, 1.0));
            float c3 = random(tile_id + glm::vec2(1.0, 1.0));

            // sample value between four corners with bilinear interpolation
            // c0 ------- c1
            //  | \	/     |
            //  |  x	  |    x: sample point
            //  | /	\     |
            // c2 ------- c3
            float m0_x = glm::mix(c0, c1, tile_pos.x);
            float m1_x = glm::mix(c2, c3, tile_pos.x);
            float m = glm::mix(m0_x, m1_x, tile_pos.y);

            static constexpr float threshold = 0.5f;
            map_data[y * 64 + x] = (char)(glm::step(threshold, m) * 255.f);

            /*
            Alternate version for smooth transition

            static constexpr float threshold_a = 0.3f;
            static constexpr float threshold_b = 0.7f;
            map_data[y * 64 + x] = (char)(glm::smoothstep(threshold_a, threshold_b, m) * 255.f);
            */
        }
    }

    m_renderer.set_map_data(map_data, 64, 64);

    delete[] map_data;
}
