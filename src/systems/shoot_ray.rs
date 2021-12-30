use crate::camera::Camera;
use crate::physics::GamePhysics;
use rapier3d::geometry::ColliderHandle;
use rapier3d::math::Real;
use rapier3d::prelude::Ray;

pub const MAX_RAYCAST_DISTANCE: f32 = 1000.0;

pub fn shoot_ray(physics: &GamePhysics, camera: &Camera) -> Option<(ColliderHandle, Real)> {
    let ray = Ray::new(
        nalgebra::Point::from(camera.position + camera.get_direction().into_inner() * 1.0),
        camera.get_direction().into_inner(),
    );
    physics.query_pipeline.cast_ray(
        &physics.collider_set,
        &ray,
        MAX_RAYCAST_DISTANCE,
        true,
        physics.interaction_groups,
        None,
    )
}
