use crate::physics::GamePhysics;
use nalgebra::Point3;
use rapier3d::prelude::*;

pub fn setup_player_collider(physics: &mut GamePhysics) -> RigidBodyHandle {
    let player_rigid_body_handle = physics.rigid_body_set.insert(
        RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .translation(nalgebra::Vector3::new(0.0, 3.0, 0.0))
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
