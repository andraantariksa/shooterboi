#include "Engine.hpp"
#include "Camera.hpp"
#include "InputProcessor.hpp"
#include "logic/components/Transform.hpp"
#include "logic/components/Player.hpp"
#include "logic/components/Collider.hpp"
#include "logic/components/RigidBody.hpp"
#include "logic/custom-components/player/AudioSourceShoot.hpp"
#include "logic/custom-components/player/AudioSourceShooted.hpp"
#include "utils/Converter.hpp"

#include <random>

Engine::Engine() :
    m_physics_world(nullptr)
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

    //auto* sound_shoot = new SoLoud::Wav;
    //sound_shoot->load("../../../assets/audio/shoot.wav");

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

    /*
    auto base_terrain_entity = m_registry.create();
    m_registry.emplace<Transform>(base_terrain_entity, glm::vec3(0.0f));
    m_registry.emplace<RigidBody>(
        base_terrain_entity,
        m_physics_world,
        m_registry.get<Transform>(base_terrain_entity),
        reactphysics3d::BodyType::STATIC,
        std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> {
        std::make_pair(
            m_physics_common.createBoxShape(reactphysics3d::Vector3(
                100.0f,
                0.0001f,
                100.0f)),
            reactphysics3d::Transform()) }
    );
    */

    std::mt19937 gen;
    std::uniform_real_distribution<float> dis(1.0f, 10.0f);

    for (uint32_t i = 0; i < 10; i++) {
        auto test_sphere = m_registry.create();
        m_registry.emplace<Transform>(test_sphere, glm::vec3(dis(gen), dis(gen), dis(gen)));
        m_registry.emplace<Renderable>(
            test_sphere,
            RenderObjectType::Object,
            ShapeType::Sphere,
            ShapeOperator::Union,
            glm::vec3(glm::vec3(46.f, 209.f, 162.f) / 255.f));

        // set shape data
        auto& test_sphere_renderable = m_registry.get<Renderable>(test_sphere);
        test_sphere_renderable.sh_sphere.radius = 0.5f;
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
}

AudioResourceID Engine::load_audio_resource(const char* path)
{
    auto res = new SoLoud::Wav();
    res->load(path);
    m_audio_resources.push_back(res);
    return m_audio_res_id_counter++;
}

void Engine::update(float dt, const InputProcessor& input_processor)
{
    //auto camera_direction = camera.get_direction();
    //auto camera_direction_horizontal = camera.get_direction_without_pitch();

    // update player movement
    {
        auto& transform = m_registry.get<Transform>(m_player_entity);
        auto& rigid_body = m_registry.get<RigidBody>(m_player_entity);
        auto& player = m_registry.get<Player>(m_player_entity);

        if (input_processor.is_mouse_moving()) {
            player.move_direction(input_processor.get_mouse_acc());
        }

        auto camera_direction_horizontal = player.get_direction();

        if (input_processor.is_action_key_down(ActionKey::MoveForward)) {
            transform.m_position += camera_direction_horizontal * 3.0f * dt;
        }
        else if (input_processor.is_action_key_down(ActionKey::MoveBackward)) {
            transform.m_position -= camera_direction_horizontal * 3.0f * dt;
        }

        if (input_processor.is_action_key_down(ActionKey::MoveRight)) {
            transform.m_position += glm::normalize(glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal)) * 3.0f * dt;
        }
        else if (input_processor.is_action_key_down(ActionKey::MoveLeft)) {
            transform.m_position -= glm::normalize(glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal)) * 3.0f * dt;
        }
    }

    /*
    auto player_view = m_registry.view<Player, Transform, RigidBody>();
    for (const auto& entity : player_view) {
        auto& transform = player_view.get<Transform>(entity);
        auto& rigid_body = player_view.get<RigidBody>(entity);
        auto& player = player_view.get<Player>(entity);

        auto camera_direction_horizontal = player.get_direction_without_pitch();

        if (input_processor.is_action_key_down(ActionKey::MoveForward)) {
            transform.m_position += camera_direction_horizontal * 3.0f * dt;
        }
        else if (input_processor.is_action_key_down(ActionKey::MoveBackward)) {
            transform.m_position -= camera_direction_horizontal * 3.0f * dt;
        }

        if (input_processor.is_action_key_down(ActionKey::MoveRight)) {
            transform.m_position += glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal) * 3.0f * dt;
        }
        else if (input_processor.is_action_key_down(ActionKey::MoveLeft)) {
            transform.m_position -= glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal) * 3.0f * dt;
        }

        if (input_processor.is_action_key_down(ActionKey::Jump)) {
            rigid_body.getRigidBody()->setLinearVelocity(reactphysics3d::Vector3(0.0f, 5.0f, 0.0f));
        }

        //camera.m_position = transform.m_position;
    }*/

    /*
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

    {
        auto collider_view = m_registry.view<Collider, Transform>();
        for (const auto& entity : collider_view) {
            auto& transform = collider_view.get<Transform>(entity);
            auto& collider = collider_view.get<Collider>(entity);

            const auto& collider_transform = collider.getTransform();
            const auto& collider_position = collider_transform.getPosition();
            transform.m_position = glm::vec3(collider_position.x, collider_position.y, collider_position.z);
        }
    }

    auto player_shoot_audio_view = m_registry.view<Player, Transform, AudioSourceShoot, AudioSourceShooted>();
    for (const auto& entity : player_shoot_audio_view) {
        auto& audio_shoot = player_shoot_audio_view.get<AudioSourceShoot>(entity);
        auto& audio_shooted = player_shoot_audio_view.get<AudioSourceShooted>(entity);
        auto& transform = player_shoot_audio_view.get<Transform>(entity);
        if (input_processor.is_mouse_pressed(MouseButton::Left)) {
            audio_shoot.play(m_soloud);

            glm::vec3 ray_from = transform.m_position;

            class MyCallbackClass : public reactphysics3d::RaycastCallback {
            public:
                virtual reactphysics3d::decimal notifyRaycastHit(const reactphysics3d::RaycastInfo& info) {
                    // Display the world hit point coordinates 
                    std::cout << "Hit point : " <<
                        info.worldPoint.x <<
                        info.worldPoint.y <<
                        info.worldPoint.z <<
                        std::endl;

                    std::cout << "Object pos " <<
                        info.body->getTransform().getPosition().x <<
                        info.body->getTransform().getPosition().y <<
                        info.body->getTransform().getPosition().z << std::endl;

                    // Return a fraction of 1.0 to gather all hits 
                    return reactphysics3d::decimal(1.0);
                }
            };
            MyCallbackClass a;

            reactphysics3d::Ray shoot_ray(to_react(ray_from), to_react(ray_from + camera_direction * 10.0f));
            physic_world->raycast(shoot_ray, &a);
        }
    }

    auto render_object_view = m_registry.view<RenderObject>();
    for (const auto& entity : render_object_view) {
        auto& render_object = render_object_view.get<RenderObject>(entity);
        const uint32_t index = render_object.get_render_objects_index();
        RenderObjectData& render_object_data = render_objects.get(index);
        switch (render_object_data.m_type) {
            case RenderObjectDataType::Enemy: {
                auto& transform = m_registry.get<Transform>(entity);
                render_object_data.m_data.enemies.transform.m_position.x = transform.m_position.x;
                render_object_data.m_data.enemies.transform.m_position.y = transform.m_position.y;
                render_object_data.m_data.enemies.transform.m_position.z = transform.m_position.z;
            }
            break;
            default:
                break;
        }
    }

    {
        auto collider_view = m_registry.view<RigidBody, Transform>();
        for (const auto& entity : collider_view) {
            auto& transform = collider_view.get<Transform>(entity);
            auto& collider = collider_view.get<RigidBody>(entity);

            collider.setTransform(transform);
        }
    }*/
}

void Engine::render_scene(const glm::vec2& resolution)
{
    m_renderer.begin();
    
    auto renderable_view = m_registry.view<Renderable, Transform>();
    for (const auto& entity : renderable_view) {
        // TODO: Add frustum culling
        m_renderer.submit(
            renderable_view.get<Transform>(entity),
            renderable_view.get<Renderable>(entity));
    }

    m_renderer.end();

    auto& player_transform = m_registry.get<Transform>(m_player_entity);
    auto& player = m_registry.get<Player>(m_player_entity);
    auto player_dir = player.get_direction();

    m_renderer.render(
        0.0f,
        resolution,
        player_transform.m_position,
        player_dir);
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
