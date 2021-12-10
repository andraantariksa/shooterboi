use crate::animation::InOutAnimation;
use crate::enemy::{HasMaterial, HITTED_MATERIAL_DURATION};

use crate::renderer::render_objects::MaterialType;

use nalgebra::{Unit, Vector3};


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
    dir: Vector3<f32>,
    material_state: EnemyMaterialState,
}

impl Gunman {
    pub fn new() -> Self {
        Self {
            state: EnemyState::FollowInvisible,
            rotation: 0.0,
            dir: Vector3::new(0.0, 1.0, 0.0),
            material_state: EnemyMaterialState::None,
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        obj_pos: &mut Vector3<f32>,
        player_pos: &Vector3<f32>,
    ) {
        match self.material_state {
            EnemyMaterialState::Hitted(ref mut duration) => {
                *duration -= delta_time;
                if *duration <= 0.0f32 {
                    self.material_state = EnemyMaterialState::None
                }
            }
            _ => {}
        };

        let rotation_speed = 5.0; // Change this, if you feel the enemy rotation is too slow.
        let current_dir = Unit::new_normalize(*player_pos - *obj_pos);

        // This can be normalized. However, since the player
        // can't move fast, the enemy rotation velocity wouldn't
        // change much, so the normalized version of this vector is optional.
        let delta_dir = Unit::new_normalize(*current_dir - self.dir);

        let mut is_moving = false;

        if delta_dir.x.abs() > 0.01 {
            self.dir.x += delta_dir.x * delta_time * rotation_speed;
            is_moving |= true;
        }

        if delta_dir.z.abs() > 0.01 {
            self.dir.z += delta_dir.z * delta_time * rotation_speed;
            is_moving |= true;
        }

        if is_moving {
            self.rotation = self.dir.x.atan2(self.dir.z);
        }
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        self.dir
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
