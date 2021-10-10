#ifndef _SRC_UTILS_CONVERTER_HPP
#define _SRC_UTILS_CONVERTER_HPP

#include <reactphysics3d/reactphysics3d.h>
#include <glm/glm.hpp>

inline reactphysics3d::Vector3 to_react(glm::vec3 x) {
    return reactphysics3d::Vector3(x.x, x.y, x.z);
}

#endif
