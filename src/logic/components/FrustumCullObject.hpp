#ifndef _SRC_FRUSTUM_CULL_OBJECT_HPP
#define _SRC_FRUSTUM_CULL_OBJECT_HPP

#include "Frustum.hpp"

class FrustumCullObject
{
private:
    float m_radius;
public:
    FrustumCullObject(float radius) :
        m_radius(radius)
    {
    }

    bool is_inside_frustum(const glm::vec3& position, const Frustum& frustum) const
    {
        bool temp;
        ImGui::Begin("Frustum Check");
        temp = frustum.bottom_plane.get_signed_distance(position) > -m_radius;
        ImGui::Checkbox("Bottom", &temp);
        temp = frustum.top_plane.get_signed_distance(position) > -m_radius;
        ImGui::Checkbox("Top", &temp);
        temp = frustum.right_plane.get_signed_distance(position) > -m_radius;
        ImGui::Checkbox("Right", &temp);
        temp = frustum.left_plane.get_signed_distance(position) > -m_radius;
        ImGui::Checkbox("Left", &temp);
        temp = frustum.near_plane.get_signed_distance(position) > -m_radius;
        ImGui::Checkbox("Near", &temp);
        temp = frustum.far_plane.get_signed_distance(position) > -m_radius;
        ImGui::Checkbox("Far", &temp);
        ImGui::End();
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