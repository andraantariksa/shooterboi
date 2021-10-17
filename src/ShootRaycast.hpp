#ifndef _SRC_SHOOT_RAYCAST_HPP
#define _SRC_SHOOT_RAYCAST_HPP

#include <reactphysics3d/reactphysics3d.h>

class ShootRaycast : public reactphysics3d::RaycastCallback
{
private:
    entt::registry* m_registry;
    reactphysics3d::PhysicsWorld* m_physics_world;
public:
    ShootRaycast(entt::registry* registry, reactphysics3d::PhysicsWorld* physics_world) :
        m_registry(registry),
        m_physics_world(physics_world) {
    }

    virtual reactphysics3d::decimal notifyRaycastHit(const reactphysics3d::RaycastInfo& info) {
        void* user_data_ptr = info.body->getUserData();
        if (user_data_ptr) {
            RigidBodyData* user_data = static_cast<RigidBodyData*>(user_data_ptr);
            m_registry->get<RigidBody>(user_data->entity).destroy(m_physics_world);
            m_registry->destroy(user_data->entity);
        }

        return reactphysics3d::decimal(1.0);
    }
};

#endif
