#ifndef _SRC_LOGIC_COMPONENTS_COLLIDER_HPP
#define _SRC_LOGIC_COMPONENTS_COLLIDER_HPP

#include <reactphysics3d/reactphysics3d.h>
#include "Transform.hpp"

class RigidBody {
private:
	reactphysics3d::RigidBody* m_rigidbody;
public:
	RigidBody(reactphysics3d::PhysicsWorld& world, Transform& transform, reactphysics3d::BodyType body_type):
		m_rigidbody(world.createRigidBody(transform.to_react_transform())) {
		m_rigidbody->setType(body_type);
	}
	
	inline void destroy(reactphysics3d::PhysicsWorld& world) { world.destroyRigidBody(m_rigidbody); };

	inline void setTransform(Transform& transform) { m_rigidbody->setTransform(transform.to_react_transform()); }
	inline void getTransform(Transform& transform) { m_rigidbody->getTransform(); }
};

#endif
