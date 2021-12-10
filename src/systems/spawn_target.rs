use crate::entity::target::Target;
use crate::physics::GamePhysics;
use hecs::World;
use nalgebra::Vector3;
use rapier3d::prelude::{ColliderBuilder, SharedShape};

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
