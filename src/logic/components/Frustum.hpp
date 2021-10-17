#ifndef _SRC_FRUSTUM_HPP
#define _SRC_FRUSTUM_HPP

#include <glm/glm.hpp>

class FrustumPlane
{
public:
    glm::vec3 m_normal;
    float m_distance;

    FrustumPlane() = default;

    FrustumPlane(const glm::vec3& p1, const glm::vec3& normal) :
        m_normal(glm::normalize(normal)),
        m_distance(glm::dot(normal, p1))
    {
    }

    float get_signed_distance(const glm::vec3& point) const
    {
        return glm::dot(m_normal, point) - m_distance;
    }
};

class Frustum
{
public:
    FrustumPlane top_plane;
    FrustumPlane bottom_plane;
    FrustumPlane right_plane;
    FrustumPlane left_plane;
    FrustumPlane far_plane;
    FrustumPlane near_plane;
};

#endif
