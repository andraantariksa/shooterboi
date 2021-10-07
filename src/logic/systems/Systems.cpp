#include <entt/entt.hpp>
#include <iostream>
#include <memory>

#include "../../InputProcessor.h"
#include "../components/Transform.hpp"
#include "../components/Player.hpp"
#include "../../Camera.hpp"
#include "../components/Collider.hpp"
#include "../custom-components/player/AudioSourceShoot.hpp"
#include <soloud_wav.h>

void init(entt::registry& registry, SoLoud::Soloud& soloud) {
    auto* sound_shoot = new SoLoud::Wav;
    sound_shoot->load("../../../assets/audio/shoot.wav");

    auto player_entity = registry.create();
    registry.emplace<Player>(player_entity);
    registry.emplace<Transform>(player_entity, glm::vec3(glm::vec3(8.0f, 3.0f, -8.0f)));
    registry.emplace<AudioSourceShoot>(player_entity, soloud, std::unique_ptr<SoLoud::AudioSource>(sound_shoot));
}

void update(entt::registry& registry, float delta_time, InputProcessor& input_processor, Camera& camera, SoLoud::Soloud& soloud) {
    auto camera_direction = camera.get_direction();
    auto camera_direction_horizontal = camera.get_direction_without_pitch();

    auto player_view = registry.view<Player, Transform>();
    for (const auto& entity : player_view) {
        auto& transform = player_view.get<Transform>(entity);

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

        camera.m_position = transform.m_position;
    }

    auto player_shoot_audio_view = registry.view<Player, AudioSourceShoot>();
    for (const auto& entity : player_shoot_audio_view) {
        auto& audio = player_shoot_audio_view.get<AudioSourceShoot>(entity);
        if (input_processor.is_mouse_pressed(MouseButton::Left)) {
            audio.play(soloud);
        }
    }
}
