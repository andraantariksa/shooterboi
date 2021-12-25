use crate::entity::target::Target;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::{MaterialType, ShapeType};
use crate::renderer::Renderer;
use hecs::World;
use nalgebra::Vector3;
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
