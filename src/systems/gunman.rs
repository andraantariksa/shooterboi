use crate::entity::enemy::gunman::{Bullet, Gunman, GunmanOp, BULLET_RAD, BULLET_SPEED};
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
        ColliderBuilder::new(SharedShape::cuboid(
            0.3 * 3.1 * 0.2,
            1.4 * 1.4 * 0.2,
            0.4 * 3.0 * 0.2,
        ))
        .user_data(entity.to_bits().get() as u128)
        .build(),
        rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    // Arms
    // Left
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::capsule(
            Point3::<f32>::new(1.1, 1.4, 0.0) * 0.2,
            Point3::<f32>::new(1.1, 1.4 - 2.3, 0.0) * 0.2,
            0.4 * 0.2,
        ))
        .user_data(entity.to_bits().get() as u128)
        .build(),
        rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    // Right
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::capsule(
            Point3::<f32>::new(-1.1, 1.6, 0.0) * 0.2,
            Point3::<f32>::new(-1.1, 1.6, 2.3) * 0.2,
            0.4 * 0.2,
        ))
        .user_data(entity.to_bits().get() as u128)
        .build(),
        rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    // Head
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::ball(0.2))
            .translation(Vector3::new(0.0, 3.1, 0.0) * 0.2)
            .user_data(entity.to_bits().get() as u128)
            .build(),
        rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    // Legs
    // Left
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::capsule(
            Point3::<f32>::new(0.5, -2.0, 0.0) * 0.2,
            Point3::<f32>::new(0.5, -2.0 - 2.5, 0.0) * 0.2,
            0.4 * 0.2,
        ))
        .user_data(entity.to_bits().get() as u128)
        .build(),
        rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    // Right
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::capsule(
            Point3::<f32>::new(-0.5, -2.0, 0.0) * 0.2,
            Point3::<f32>::new(-0.5, -2.0 - 2.5, 0.0) * 0.2,
            0.4 * 0.2,
        ))
        .user_data(entity.to_bits().get() as u128)
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
        objects.scale = 0.2;
        objects.shape_type_material_ids.0 = ShapeType::Gunman;
        objects.shape_type_material_ids.1 = gunman.get_material();
        objects.shape_type_material_ids.2 = MaterialType::Black;
        objects.rotation = rb.rotation().inverse().to_homogeneous();

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
    let mut gunman_ops = Vec::new();
    for (_id, (gunman, rb_handle)) in world.query_mut::<(&mut Gunman, &RigidBodyHandle)>() {
        let gunman_rigid_body = physics.rigid_body_set.get_mut(*rb_handle).unwrap();
        let mut gunman_pos = *gunman_rigid_body.translation();
        gunman_ops.push(gunman.update(rng, delta_time, &mut gunman_pos, player_position));
        gunman_rigid_body.set_translation(gunman_pos, true);
        gunman_rigid_body.set_rotation(gunman.get_rotation(), true);
    }

    for op in gunman_ops {
        match op {
            GunmanOp::None => {}
            GunmanOp::Shoot { pos, mut dir } => {
                dir.y = 0.0;
                spawn_bullet(world, physics, pos + dir * 1.0, dir);
            }
        }
    }
}

pub fn spawn_bullet(
    world: &mut World,
    physics: &mut GamePhysics,
    pos: Vector3<f32>,
    dir: Vector3<f32>,
) {
    let entity = world.reserve_entity();
    let rigid_body_handle = physics.rigid_body_set.insert(
        RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .user_data(entity.to_bits().get() as u128)
            .translation(pos)
            .linvel(dir * BULLET_SPEED)
            .build(),
    );
    physics.collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::ball(BULLET_RAD))
            .user_data(entity.to_bits().get() as u128)
            .active_events(ActiveEvents::CONTACT_EVENTS)
            .build(),
        rigid_body_handle,
        &mut physics.rigid_body_set,
    );
    world.spawn_at(entity, (Bullet, rigid_body_handle));
}

pub fn enqueue_bullet(world: &mut World, physics: &mut GamePhysics, renderer: &mut Renderer) {
    for (_id, (bullet, rb_handle)) in world.query_mut::<(&Bullet, &RigidBodyHandle)>() {
        let rb = physics.rigid_body_set.get(*rb_handle).unwrap();
        let collider = physics
            .collider_set
            .get(rb.colliders()[0])
            .unwrap()
            .shape()
            .as_ball()
            .unwrap();

        let (objects, ref mut bound) = renderer.render_objects.next();
        objects.position = *rb.translation();
        objects.shape_data1.x = collider.radius;
        objects.shape_type_material_ids.0 = ShapeType::Sphere;
        objects.shape_type_material_ids.1 = bullet.get_material();
        objects.rotation = rb.rotation().inverse().to_homogeneous();

        *bound = ObjectBound::Sphere(collider.radius);
    }
}
