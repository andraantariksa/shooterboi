use crate::entity::Crate;
use crate::frustum::ObjectBound;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::{MaterialType, ShapeType};
use crate::renderer::Renderer;
use hecs::World;
use nalgebra::Vector3;
use rapier3d::prelude::*;

pub fn spawn_crate(
    world: &mut World,
    physics: &mut GamePhysics,
    translation: Vector3<f32>,
    size: Vector3<f32>,
) {
    let entity = world.reserve_entity();
    let rb_handle = physics.rigid_body_set.insert(
        RigidBodyBuilder::new(RigidBodyType::Static)
            .translation(translation)
            .lock_rotations()
            .lock_translations()
            .user_data(entity.to_bits() as u128)
            .build(),
    );
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::cuboid(size.x, size.y, size.z))
            .user_data(entity.to_bits() as u128)
            .build(),
        rb_handle,
        &mut physics.rigid_body_set,
    );
    world.spawn_at(
        entity,
        (
            Crate,
            rb_handle,
            ObjectBound::Sphere((size.x.powf(2.0) + size.y.powf(2.0) + size.z.powf(2.0)).sqrt()),
        ),
    );
}

pub fn enqueue_crate(world: &mut World, physics: &mut GamePhysics, renderer: &mut Renderer) {
    for (_id, (_crate, rb_handle, object_bound)) in
        world.query_mut::<(&Crate, &RigidBodyHandle, &ObjectBound)>()
    {
        let rb = physics.rigid_body_set.get(*rb_handle).unwrap();
        let collider = physics.collider_set.get(rb.colliders()[0]).unwrap();

        let (objects, ref mut bound) = renderer.render_objects.next();
        objects.position = *rb.translation();
        objects.shape_type_material_ids.0 = ShapeType::Box;
        objects.shape_type_material_ids.1 = MaterialType::Crate;
        objects.rotation = rb.rotation().to_homogeneous();

        let shape = collider.shape().as_cuboid().unwrap();
        objects.shape_data1.x = shape.half_extents.x;
        objects.shape_data1.y = shape.half_extents.y;
        objects.shape_data1.z = shape.half_extents.z;

        *bound = object_bound.clone();
    }
}
