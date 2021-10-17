#include <entt/entt.hpp>
#include <iostream>
#include <memory>

#include "../../InputProcessor.hpp"
#include "../components/Transform.hpp"
#include "../components/Player.hpp"
#include "../../Camera.hpp"
#include "../components/Collider.hpp"
#include "../components/RigidBody.hpp"
#include "../custom-components/player/AudioSourceShoot.hpp"
#include "../custom-components/player/AudioSourceShooted.hpp"
#include <soloud_wav.h>
#include "../components/RenderObject.hpp"
#include "../../RenderObjects.hpp"
#include "../../utils/Converter.hpp"

void init(
    entt::registry& registry,
    SoLoud::Soloud& soloud,
    reactphysics3d::PhysicsWorld* physic_world,
    reactphysics3d::PhysicsCommon& physic_common,
    RenderObjects<100>& render_objects) {
    
}

void update(
    entt::registry& registry,
    float delta_time,
    InputProcessor& input_processor,
    Camera& camera,
    SoLoud::Soloud& soloud,
    reactphysics3d::PhysicsWorld* physic_world,
    reactphysics3d::PhysicsCommon& physic_common,
    RenderObjects<100>& render_objects) {
    auto camera_direction = camera.get_direction();
    auto camera_direction_horizontal = camera.get_direction_without_pitch();

    {
        auto collider_view = registry.view<RigidBody, Transform>();
        for (const auto& entity : collider_view) {
            auto& transform = collider_view.get<Transform>(entity);
            auto& collider = collider_view.get<RigidBody>(entity);

            const auto& collider_transform = collider.getTransform();
            const auto& collider_position = collider_transform.getPosition();
            transform.m_position = glm::vec3(collider_position.x, collider_position.y, collider_position.z);
        }
    }
    {
        auto collider_view = registry.view<Collider, Transform>();
        for (const auto& entity : collider_view) {
            auto& transform = collider_view.get<Transform>(entity);
            auto& collider = collider_view.get<Collider>(entity);

            const auto& collider_transform = collider.getTransform();
            const auto& collider_position = collider_transform.getPosition();
            transform.m_position = glm::vec3(collider_position.x, collider_position.y, collider_position.z);
        }
    }

    auto player_view = registry.view<Player, Transform, RigidBody>();
    for (const auto& entity : player_view) {
        auto& transform = player_view.get<Transform>(entity);
        auto& rigid_body = player_view.get<RigidBody>(entity);

        if (input_processor.is_action_key_down(ActionKey::MoveForward)) {
            transform.m_position += camera_direction_horizontal * 3.0f * delta_time;
        }
        else if (input_processor.is_action_key_down(ActionKey::MoveBackward)) {
            transform.m_position -= camera_direction_horizontal * 3.0f * delta_time;
        }

        if (input_processor.is_action_key_down(ActionKey::MoveRight)) {
            transform.m_position += glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal) * 3.0f * delta_time;
        }
        else if (input_processor.is_action_key_down(ActionKey::MoveLeft)) {
            transform.m_position -= glm::cross(glm::vec3(0.0f, 1.0f, 0.0f), camera_direction_horizontal) * 3.0f * delta_time;
        }

        if (input_processor.is_action_key_down(ActionKey::Jump)) {
            rigid_body.getRigidBody()->setLinearVelocity(reactphysics3d::Vector3(0.0f, 5.0f, 0.0f));
        }

        camera.m_position = transform.m_position;
    }

    auto player_shoot_audio_view = registry.view<Player, Transform, AudioSourceShoot, AudioSourceShooted>();
    for (const auto& entity : player_shoot_audio_view) {
        auto& audio_shoot = player_shoot_audio_view.get<AudioSourceShoot>(entity);
        auto& audio_shooted = player_shoot_audio_view.get<AudioSourceShooted>(entity);
        auto& transform = player_shoot_audio_view.get<Transform>(entity);
        if (input_processor.is_mouse_pressed(MouseButton::Left)) {
            //audio_shoot.play(soloud);

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

    auto render_object_view = registry.view<RenderObject>();
    for (const auto& entity : render_object_view) {
        auto& render_object = render_object_view.get<RenderObject>(entity);
        const uint32_t index = render_object.get_render_objects_index();
        RenderObjectData& render_object_data = render_objects.get(index);
        switch (render_object_data.m_type) {
        case RenderObjectDataType::Enemy: {
            auto& transform = registry.get<Transform>(entity);
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
        auto collider_view = registry.view<RigidBody, Transform>();
        for (const auto& entity : collider_view) {
            auto& transform = collider_view.get<Transform>(entity);
            auto& collider = collider_view.get<RigidBody>(entity);

            collider.setTransform(transform);
        }
    }
}
