use crate::animation::InOutAnimation;
use crate::entity::enemy::{HITTED_MATERIAL_DURATION, ROTATION_SPEED};
use crate::entity::HasMaterial;
use crate::renderer::render_objects::MaterialType;
use crate::timer::Timer;
use nalgebra::{Unit, Vector3};

pub enum EnemyState {
    Attack(InOutAnimation),
    Follow,
}

pub enum EnemyMaterialState {
    None,
    Hitted(Timer),
}

pub struct Swordman {
    state: EnemyState,
    rotation: f32,
    dir: Vector3<f32>,
    material_state: EnemyMaterialState,
}

impl Swordman {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Follow,
            rotation: 0.0,
            material_state: EnemyMaterialState::None,
            dir: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        obj_pos: &mut Vector3<f32>,
        player_pos: &Vector3<f32>,
    ) {
        match self.material_state {
            EnemyMaterialState::Hitted(ref mut timer) => {
                timer.update(delta_time);
                if timer.is_finished() {
                    self.material_state = EnemyMaterialState::None;
                }
            }
            _ => {}
        };

        let current_dir = Unit::new_normalize(*player_pos - *obj_pos);

        // This can be normalized. However, since the player
        // can't move fast, the enemy rotation velocity wouldn't
        // change much, so the normalized version of this vector is optional.
        let delta_dir = Unit::new_normalize(*current_dir - self.dir);

        let mut is_moving = false;

        if delta_dir.x.abs() > 0.01 {
            self.dir.x += delta_dir.x * delta_time * ROTATION_SPEED;
            is_moving |= true;
        }

        if delta_dir.z.abs() > 0.01 {
            self.dir.z += delta_dir.z * delta_time * ROTATION_SPEED;
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

    pub fn hitanim(&self) -> f32 {
        match self.state {
            EnemyState::Attack(ref anim) => anim.get_value(),
            _ => 0.0,
        }
    }

    pub fn hit(&mut self) {
        self.material_state = EnemyMaterialState::Hitted(Timer::new(HITTED_MATERIAL_DURATION));
    }
}

impl HasMaterial for Swordman {
    fn get_material(&self) -> MaterialType {
        match self.material_state {
            EnemyMaterialState::None => MaterialType::White,
            EnemyMaterialState::Hitted(_) => MaterialType::Red,
        }
    }
}