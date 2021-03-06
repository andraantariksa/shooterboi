use crate::entity::target::SphereTarget;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::ShapeType;
use crate::renderer::Renderer;
use hecs::World;
use nalgebra::Vector3;
use rand::prelude::SmallRng;
use rapier3d::prelude::{ColliderBuilder, ColliderHandle, SharedShape};

pub fn spawn_target(
    world: &mut World,
    physics: &mut GamePhysics,
    pos: Vector3<f32>,
    target: SphereTarget,
) {
    let entity = world.reserve_entity();
    world.spawn_at(
        entity,
        (
            physics.collider_set.insert(
                ColliderBuilder::new(SharedShape::ball(0.5))
                    .user_data(entity.to_bits().get() as u128)
                    .translation(pos)
                    .build(),
            ),
            target,
        ),
    );
}

#[allow(clippy::never_loop)]
pub fn is_any_target_exists(world: &mut World) -> bool {
    for (_, _) in world.query_mut::<&SphereTarget>() {
        return true;
    }
    false
}

pub fn enqueue_target(world: &mut World, physics: &mut GamePhysics, renderer: &mut Renderer) {
    for (_id, (collider_handle, target)) in world.query_mut::<(&ColliderHandle, &SphereTarget)>() {
        let collider = physics.collider_set.get(*collider_handle).unwrap();

        let (objects, ref mut bound) = renderer.render_objects.next();
        objects.position = *collider.translation();
        objects.shape_type_material_ids.0 = ShapeType::Sphere;
        objects.shape_type_material_ids.1 = target.get_material();
        objects.rotation = collider.rotation().to_homogeneous();

        let shape = collider.shape().as_ball().unwrap();
        objects.shape_data1.x = shape.radius;

        *bound = objects.get_bounding_sphere_radius();
    }
}

pub fn update_target(
    world: &mut World,
    physics: &mut GamePhysics,
    delta_time: f32,
    _rng: &mut SmallRng,
) {
    for (_id, (target, collider_handle)) in
        world.query_mut::<(&mut SphereTarget, &ColliderHandle)>()
    {
        let target_collider = physics.collider_set.get_mut(*collider_handle).unwrap();
        let mut target_pos = *target_collider.translation();
        target.update(delta_time, &mut target_pos);
        target_collider.set_translation(target_pos);
    }
}
