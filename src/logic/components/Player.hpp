#ifndef _SRC_LOGIC_COMPONENTS_PLAYER_HPP
#define _SRC_LOGIC_COMPONENTS_PLAYER_HPP

class Player
{
    static constexpr float sensitivity = 0.3f;

    float m_yaw;
    float m_pitch;
    int m_health;

public:
    Player() :
        m_yaw(90.0f),
        m_pitch(0.0f),
        m_health(10)
    {
    }

    void move_direction(const glm::vec2& offset)
    {
        glm::vec2 ofs = offset * sensitivity;
        m_yaw -= ofs.x;
        m_pitch -= ofs.y;

        m_pitch = glm::clamp(m_pitch, -89.0f, 89.0f);
    }

    glm::vec3 get_direction_without_pitch()
    {
        glm::vec3 direction;
        direction.x = std::cos(glm::radians(m_yaw));
        direction.y = 0.0f;
        direction.z = std::sin(glm::radians(m_yaw));

        return glm::normalize(direction);
    }

    glm::vec3 get_direction()
    {
        glm::vec3 direction;
        direction.x = std::cos(glm::radians(m_yaw)) * std::cos(glm::radians(m_pitch));
        direction.y = std::sin(glm::radians(m_pitch));
        direction.z = std::sin(glm::radians(m_yaw)) * std::cos(glm::radians(m_pitch));

        return glm::normalize(direction);
    }
};

#endif