use crate::entity::target::Target;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::ShapeType;
use crate::renderer::Renderer;
use hecs::World;
use nalgebra::{Matrix4, Vector3};
use rand::prelude::SmallRng;
use rapier3d::prelude::{ColliderBuilder, ColliderHandle, SharedShape};

pub fn spawn_target(
    world: &mut World,
    physics: &mut GamePhysics,
    pos: Vector3<f32>,
    target: Target,
) {
    let entity = world.reserve_entity();
    world.spawn_at(
        entity,
        (
            physics.collider_set.insert(
                ColliderBuilder::new(SharedShape::ball(0.5))
                    .user_data(entity.to_bits() as u128)
                    .translation(pos)
                    .build(),
            ),
            target,
        ),
    );
}

pub fn is_any_target_exists(world: &mut World) -> bool {
    let mut exists = false;
    for (_, (_)) in world.query_mut::<(&Target)>() {
        exists = true;
        break;
    }
    exists
}

pub fn enqueue_target(world: &mut World, physics: &mut GamePhysics, renderer: &mut Renderer) {
    for (_id, (collider_handle, target)) in world.query_mut::<(&ColliderHandle, &Target)>() {
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
    rng: &mut SmallRng,
) {
    for (_id, (target, collider_handle)) in world.query_mut::<(&mut Target, &ColliderHandle)>() {
        let target_collider = physics.collider_set.get_mut(*collider_handle).unwrap();
        let mut target_pos = *target_collider.translation();
        target.update(delta_time, &mut target_pos);
        target_collider.set_translation(target_pos);
    }
}
