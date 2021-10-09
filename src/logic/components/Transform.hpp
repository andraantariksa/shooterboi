#ifndef _SRC_LOGIC_COMPONENTS_TRANSFORM_HPP
#define _SRC_LOGIC_COMPONENTS_TRANSFORM_HPP

#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp>
#include <reactphysics3d/reactphysics3d.h>

class Transform {
public:
	glm::vec3 m_position;
	//glm::quat m_rotation;

	Transform(glm::vec3& position = glm::vec3(), glm::quat& rotation = glm::quat()) :
		m_position(position)
	//	m_rotation(rotation)
	{}

	reactphysics3d::Transform to_react_transform() {
		return reactphysics3d::Transform(
			reactphysics3d::Vector3(m_position.x, m_position.y, m_position.z),
			reactphysics3d::Quaternion::identity()); // m_rotation.x, m_rotation.y, m_rotation.z, m_rotation.w));
	}
};

#endif
