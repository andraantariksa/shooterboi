#ifndef _SRC_LOGIC_COMPONENTS_PLAYER_HPP
#define _SRC_LOGIC_COMPONENTS_PLAYER_HPP

#include <glm/glm.hpp>
#include <glm/gtx/io.hpp>
#include "Frustum.hpp"

class Player
{
    static constexpr float sensitivity = 0.5f;
    static constexpr glm::vec3 world_up = glm::vec3(0.0f, 1.0f, 0.0f);
    static constexpr float shoot_interval = 1.0f;
    static constexpr float fov_angle = glm::radians(90.0f);
    static constexpr float z_far = 100.0f;
    static constexpr float z_near = 0.001f;

    std::chrono::steady_clock::time_point m_last_time_shoot;

    float m_yaw;
    float m_pitch;
    int m_health;

public:
    Player() :
        m_yaw(90.0f),
        m_pitch(0.0f),
        m_health(10),
        m_last_time_shoot(std::chrono::high_resolution_clock::now())
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

    glm::vec3 get_direction() const
    {
        glm::vec3 direction;
        direction.x = std::cos(glm::radians(m_yaw)) * std::cos(glm::radians(m_pitch));
        direction.y = std::sin(glm::radians(m_pitch));
        direction.z = std::sin(glm::radians(m_yaw)) * std::cos(glm::radians(m_pitch));

        return glm::normalize(direction);
    }

    inline glm::vec3 get_direction_right() const
    {
        return glm::normalize(glm::cross(get_direction(), world_up));
    }

    bool is_ready_to_shoot_and_refresh()
    {
        auto time_now = std::chrono::high_resolution_clock::now();
        float time_diff = std::chrono::duration_cast<std::chrono::duration<float>>(time_now - m_last_time_shoot).count();
        if (time_diff > shoot_interval)
        {
            m_last_time_shoot = time_now;
            return true;
        }
        return false;
    }

    Frustum get_frustum(const Transform& player_transform, float aspect_ratio) const
    {
        const auto direction = get_direction();
        const auto right = glm::normalize(glm::cross(world_up, direction));
        const auto up = glm::normalize(glm::cross(direction, right));
        //      /| B
        //   c / | a
        //    /__| C
        //   A  b
        // tan(A) = a / b
        // a = plane width
        // b = distance to plane
        const float fov = std::tanf(fov_angle * .5f);
        const float h_near = z_near * fov;
        const float w_near = h_near * aspect_ratio;
        const float h_far = z_far * fov;
        const float w_far = h_far * aspect_ratio;
        const auto h_near_up = up * h_near;
        const auto w_near_right = right * w_near;
        const auto h_far_up = up * h_far;
        const auto w_far_right = right * w_far;

        const glm::vec3 z_near_plane_pos = player_transform.m_position + direction * z_near;
        const glm::vec3 z_near_plane_top_left = z_near_plane_pos + h_near_up - w_near_right;
        const glm::vec3 z_near_plane_top_right = z_near_plane_pos + h_near_up + w_near_right;
        const glm::vec3 z_near_plane_bottom_left = z_near_plane_pos - h_near_up - w_near_right;
        const glm::vec3 z_near_plane_bottom_right = z_near_plane_pos - h_near_up + w_near_right;

        const glm::vec3 z_far_plane_pos = player_transform.m_position + direction * z_far;
        const glm::vec3 z_far_plane_top_left = z_far_plane_pos + h_far_up - w_far_right;
        const glm::vec3 z_far_plane_top_right = z_far_plane_pos + h_far_up + w_far_right;
        const glm::vec3 z_far_plane_bottom_left = z_far_plane_pos - h_far_up - w_far_right;
        const glm::vec3 z_far_plane_bottom_right = z_far_plane_pos - h_far_up + w_far_right;

        /*
        frustum.near_plane = { player_transform.m_position + z_near * direction, direction };
        frustum.far_plane = { player_transform.m_position + z_far_plane_pos    , -direction };
        frustum.right_plane = { player_transform.m_position                    , glm::cross(up, z_far_plane_pos - right * half_x_side) };
        frustum.left_plane = { player_transform.m_position                     , glm::cross(z_far_plane_pos + right * half_x_side, up) };
        frustum.top_plane = { player_transform.m_position                      , glm::cross(right, z_far_plane_pos + up * half_y_side) };
        frustum.bottom_plane = { player_transform.m_position                   , glm::cross(z_far_plane_pos - up * half_y_side, right) };
        */

        return {
            { z_near_plane_top_left, z_near_plane_top_right, z_far_plane_top_left },
            { z_near_plane_bottom_left, z_far_plane_bottom_left, z_near_plane_bottom_right },
            { z_near_plane_bottom_right, z_far_plane_bottom_right, z_near_plane_top_right },
            { z_near_plane_bottom_left, z_near_plane_top_left, z_far_plane_bottom_left },
            { z_far_plane_top_left, z_far_plane_top_right, z_far_plane_bottom_left },
            { z_near_plane_top_left, z_near_plane_bottom_left, z_near_plane_top_right },
        };
    }
};

#endif