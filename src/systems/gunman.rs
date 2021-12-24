use crate::entity::enemy::gunman::Gunman;
use crate::entity::HasMaterial;
use crate::frustum::ObjectBound;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::MaterialType;
use crate::renderer::render_objects::ShapeType;
use crate::renderer::Renderer;
use hecs::World;
use nalgebra::{Point3, Vector3};
use rand::prelude::SmallRng;
use rapier3d::prelude::*;

pub fn spawn_gunman(
    world: &mut World,
    physics: &mut GamePhysics,
    pos: Vector3<f32>,
    gunman: Gunman,
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
    world.spawn_at(entity, (gunman, rigid_body_handle));
}

pub fn enqueue_gunman(world: &mut World, physics: &mut GamePhysics, renderer: &mut Renderer) {
    for (_id, (gunman, rb_handle)) in world.query_mut::<(&Gunman, &RigidBodyHandle)>() {
        let rb = physics.rigid_body_set.get(*rb_handle).unwrap();

        let (objects, ref mut bound) = renderer.render_objects.next();
        objects.position = *rb.translation();
        objects.shape_data1.x = gunman.shootanim();
        objects.shape_data1.y = gunman.get_rotation();
        objects.shape_type_material_ids.0 = ShapeType::Gunman;
        objects.shape_type_material_ids.1 = gunman.get_material();
        objects.shape_type_material_ids.2 = MaterialType::Black;
        objects.rotation = rb.rotation().to_homogeneous();

        *bound = ObjectBound::Sphere(3.0);
    }
}

pub fn update_gunmans(
    world: &mut World,
    physics: &mut GamePhysics,
    delta_time: f32,
    player_position: &Vector3<f32>,
    rng: &mut SmallRng,
) {
    for (_id, (gunman, rb_handle)) in world.query_mut::<(&mut Gunman, &RigidBodyHandle)>() {
        let gunman_rigid_body = physics.rigid_body_set.get_mut(*rb_handle).unwrap();
        let mut gunman_pos = *gunman_rigid_body.translation();
        gunman.update(rng, delta_time, &mut gunman_pos, player_position);
        gunman_rigid_body.set_translation(gunman_pos, true);
    }
}
