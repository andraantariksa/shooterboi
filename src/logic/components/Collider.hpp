#ifndef _SRC_LOGIC_COMPONENTS_COLLIDER_HPP
#define _SRC_LOGIC_COMPONENTS_COLLIDER_HPP

#include <reactphysics3d/reactphysics3d.h>
#include "Transform.hpp"

class Collider {
private:
	reactphysics3d::CollisionBody* m_body;
public:
	Collider(
		reactphysics3d::PhysicsWorld* world,
		Transform& transform,
		std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> colliders = {}) :
		m_body(world->createCollisionBody(transform.to_react_transform())) {
		for (const auto& collider : colliders) {
			m_body->addCollider(collider.first, collider.second);
		}
	}

	inline void destroy(reactphysics3d::PhysicsWorld& world) { world.destroyCollisionBody(m_body); };

	inline void setTransform(Transform& transform) const { m_body->setTransform(transform.to_react_transform()); }
	inline reactphysics3d::Transform getTransform() const { return m_body->getTransform(); }
};

#endif
