use crate::animation::InOutAnimation;
use crate::enemy::{HasMaterial, HITTED_MATERIAL_DURATION};
use crate::physics::GamePhysics;
use crate::renderer::render_objects::MaterialType;
use hecs::World;
use nalgebra::{Point3, Vector3};
use rapier3d::prelude::{
    ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle, RigidBodyType, SharedShape,
};

pub enum EnemyState {
    Shoot(InOutAnimation),
    Focus(f32),
    FollowVisible(f32),
    FollowInvisible,
}

pub enum EnemyMaterialState {
    None,
    Hitted(f32),
}

pub struct Gunman {
    state: EnemyState,
    rotation: f32,
    material_state: EnemyMaterialState,
}

impl Gunman {
    pub fn new() -> Self {
        Self {
            state: EnemyState::FollowInvisible,
            rotation: 0.0,
            material_state: EnemyMaterialState::None,
        }
    }

    pub fn update(&mut self, delta_time: f32, obj_pos: &Vector3<f32>, player_pos: &Vector3<f32>) {
        match self.material_state {
            EnemyMaterialState::Hitted(ref mut duration) => {
                *duration -= delta_time;
                if *duration <= 0.0f32 {
                    self.material_state = EnemyMaterialState::None
                }
            }
            _ => {}
        };

        let x_diff = player_pos.x - obj_pos.x;
        let z_diff = player_pos.z - obj_pos.z;
        let target_angle: f32 = x_diff.atan2(z_diff);
        let delta_angle = target_angle - self.rotation;
        println!("delta_angle {}", delta_angle);
        if delta_angle > 0.01 {
            self.rotation += if delta_angle > 0.0 {
                -3.0 * delta_time
            } else {
                3.0 * delta_time
            };
            self.rotation %= std::f32::consts::PI;
        }
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn shootanim(&self) -> f32 {
        match self.state {
            EnemyState::Shoot(ref anim) => anim.get_value(),
            _ => 0.0,
        }
    }

    pub fn hit(&mut self) {
        self.material_state = EnemyMaterialState::Hitted(HITTED_MATERIAL_DURATION);
    }
}

impl HasMaterial for Gunman {
    fn get_material(&self) -> MaterialType {
        match self.material_state {
            EnemyMaterialState::None => MaterialType::White,
            EnemyMaterialState::Hitted(_) => MaterialType::Red,
        }
    }
}
