use crate::entity::enemy::swordman::Swordman;
use crate::entity::HasMaterial;
use crate::frustum::ObjectBound;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::MaterialType;
use crate::renderer::render_objects::ShapeType;
use crate::renderer::Renderer;
use hecs::World;
use nalgebra::{Point3, Vector3};
use rapier3d::prelude::*;

pub fn spawn_swordman(
    world: &mut World,
    physics: &mut GamePhysics,
    pos: Vector3<f32>,
    swordman: Swordman,
) {
    let entity = world.reserve_entity();
    let rigid_body_handle = physics.rigid_body_set.insert(
        RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .translation(pos)
            .lock_rotations()
            .build(),
    );
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::capsule(
            Point3::<f32>::new(0.0, 1.0, 0.0),
            Point3::<f32>::new(0.0, -1.0, 0.0),
            0.5,
        ))
        .user_data(entity.to_bits() as u128)
        .build(),
        rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    world.spawn_at(entity, (swordman, rigid_body_handle));
}

pub fn enqueue_swordman(world: &mut World, physics: &mut GamePhysics, renderer: &mut Renderer) {
    for (_id, (swordman, rb_handle)) in world.query_mut::<(&Swordman, &RigidBodyHandle)>() {
        let rb = physics.rigid_body_set.get(*rb_handle).unwrap();

        let (objects, ref mut bound) = renderer.render_objects.next();
        objects.position = *rb.translation();
        objects.shape_data2.y = swordman.get_rotation();
        objects.shape_type_material_ids.0 = ShapeType::Swordman;
        objects.shape_type_material_ids.1 = swordman.get_material();
        objects.shape_type_material_ids.2 = MaterialType::Green;
        objects.rotation = rb.rotation().to_homogeneous();

        *bound = ObjectBound::Sphere(3.0);
    }
}

pub fn update_swordmans(
    world: &mut World,
    physics: &mut GamePhysics,
    delta_time: f32,
    player_position: &Vector3<f32>,
) {
    for (_id, (swordman, rb_handle)) in world.query_mut::<(&mut Swordman, &RigidBodyHandle)>() {
        let swordman_rigid_body = physics.rigid_body_set.get_mut(*rb_handle).unwrap();
        let mut swordman_pos = *swordman_rigid_body.translation();
        swordman.update(delta_time, &mut swordman_pos, player_position);
        swordman_rigid_body.set_translation(swordman_pos, true);
    }
}