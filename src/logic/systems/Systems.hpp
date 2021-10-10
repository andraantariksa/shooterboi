#ifndef _SRC_LOGIC_SYSTEMS_HPP
#define _SRC_LOGIC_SYSTEMS_HPP

#include <entt/entt.hpp>

#include "../../InputProcessor.hpp"
#include "../../Camera.hpp"
#include "../../RenderObjects.hpp"

void init(
    entt::registry& registry,
    SoLoud::Soloud& soloud,
    reactphysics3d::PhysicsWorld* physic_world,
    reactphysics3d::PhysicsCommon& physic_common,
    RenderObjects<100>& render_objects);
void update(
    entt::registry& registry,
    float delta_time,
    InputProcessor& input_processor,
    Camera& camera,
    SoLoud::Soloud& soloud,
    reactphysics3d::PhysicsWorld* physic_world,
    reactphysics3d::PhysicsCommon& physic_common,
    RenderObjects<100>& render_objects);

#endif
