#ifndef _SRC_FRUSTUM_CULL_OBJECT_HPP
#define _SRC_FRUSTUM_CULL_OBJECT_HPP

#include "Frustum.hpp"

class FrustumCullObject
{
private:
    float m_radius;
public:
    FrustumCullObject(float radius) :
        m_radius(radius) {
    }

    bool is_inside_frustum(glm::vec3& position, const Frustum& frustum) const
    {
        return
            frustum.bottom_plane.get_signed_distance(position) > -m_radius &&
            frustum.top_plane.get_signed_distance(position) > -m_radius &&
            frustum.left_plane.get_signed_distance(position) > -m_radius &&
            frustum.right_plane.get_signed_distance(position) > -m_radius &&
            frustum.near_plane.get_signed_distance(position) > -m_radius &&
            frustum.far_plane.get_signed_distance(position) > -m_radius;
    }
};

#endif