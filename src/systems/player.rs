use crate::physics::GamePhysics;
use crate::renderer::Renderer;
use nalgebra::{Point3, Vector3};
use rapier3d::prelude::*;

pub fn setup_player_collider(physics: &mut GamePhysics, position: Vector3<f32>) -> RigidBodyHandle {
    let player_rigid_body_handle = physics.rigid_body_set.insert(
        RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .translation(position)
            .lock_rotations()
            .build(),
    );
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::capsule(
            Point3::<f32>::new(0.0, -1.0, 0.0),
            Point3::<f32>::new(0.0, 0.5, 0.0),
            0.5,
        ))
        .build(),
        player_rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    player_rigid_body_handle
}

pub fn init_player(
    physics: &mut GamePhysics,
    renderer: &mut Renderer,
    player_rigid_body_handle: RigidBodyHandle,
) {
    let player_rigid_body = physics
        .rigid_body_set
        .get_mut(player_rigid_body_handle)
        .unwrap();
    renderer.camera.position = *player_rigid_body.translation();
}
