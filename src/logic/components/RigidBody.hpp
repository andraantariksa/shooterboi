#ifndef _SRC_LOGIC_COMPONENTS_RIGIDBODY_HPP
#define _SRC_LOGIC_COMPONENTS_RIGIDBODY_HPP

#include <reactphysics3d/reactphysics3d.h>
#include "Transform.hpp"

enum class RigidBodyType
{
	Enemy,
};

struct RigidBodyData
{
	RigidBodyType type;
	entt::entity entity;
};

class RigidBody
{
private:
	reactphysics3d::RigidBody* m_rigidbody;
	reactphysics3d::PhysicsWorld* m_world;

public:
	RigidBody(
		reactphysics3d::PhysicsWorld* world,
		Transform& transform,
		reactphysics3d::BodyType body_type,
		std::vector<std::pair<reactphysics3d::CollisionShape*, reactphysics3d::Transform>> colliders = {}) :
		m_world(world),
		m_rigidbody(world->createRigidBody(transform.to_react_transform()))
	{
		m_rigidbody->setType(body_type);
		for (const auto& collider : colliders) {
			m_rigidbody->addCollider(collider.first, collider.second);
		}
		m_rigidbody->setLinearDamping(0.0f);
		m_rigidbody->setAngularDamping(0.0f);
	}

	~RigidBody()
	{
		destroy(m_world);
	}
	
	inline void destroy(reactphysics3d::PhysicsWorld* world) { world->destroyRigidBody(m_rigidbody); };

	inline reactphysics3d::Collider* getCollider(reactphysics3d::uint index) const { return m_rigidbody->getCollider(index); }
	inline reactphysics3d::RigidBody* getRigidBody() const { return m_rigidbody; }
	inline void setTransform(Transform& transform) const { m_rigidbody->setTransform(transform.to_react_transform()); }
	inline reactphysics3d::Transform getTransform() const { return m_rigidbody->getTransform(); }
};

template<>
struct entt::component_traits<RigidBody> : public basic_component_traits
{
	using in_place_delete = std::true_type;
};

#endif
