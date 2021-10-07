#ifndef _SRC_CAMERA_HPP
#define _SRC_CAMERA_HPP

#include <glm/glm.hpp>

class Camera {
public:
    const float sensitivity = 0.3f;

    glm::vec3 m_position;
    float m_yaw;
    float m_pitch;
    float m_roll;

    Camera(glm::vec3& position = glm::vec3(0.0f), float yaw = 0.0f, float pitch = 0.0f):
        m_position(position),
        m_yaw(yaw),
        m_pitch(pitch) {
    }

    void move_direction(glm::vec2& offset) {
        offset *= sensitivity;
        m_yaw -= offset.x;
        m_pitch -= offset.y;

        m_pitch = glm::clamp(m_pitch, -89.0f, 89.0f);
    }

    glm::vec3 get_direction_without_pitch() {
        glm::vec3 direction;
        direction.x = std::cos(glm::radians(m_yaw));
        direction.y = 0.0f;
        direction.z = std::sin(glm::radians(m_yaw));

        return glm::normalize(direction);
    }

    glm::vec3 get_direction() {
        glm::vec3 direction;
        direction.x = std::cos(glm::radians(m_yaw)) * std::cos(glm::radians(m_pitch));
        direction.y = std::sin(glm::radians(m_pitch));
        direction.z = std::sin(glm::radians(m_yaw)) * std::cos(glm::radians(m_pitch));

        return glm::normalize(direction);
    }
};

#endif