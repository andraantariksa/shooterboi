use crate::animation::InOutAnimation;
use crate::enemy::{HasMaterial, HITTED_MATERIAL_DURATION};
use crate::renderer::render_objects::MaterialType;
use nalgebra::{UnitVector3, Vector3};

pub enum EnemyState {
    Attack(InOutAnimation),
    Follow,
}

pub enum EnemyMaterialState {
    None,
    Hitted(f32),
}

pub struct Swordman {
    state: EnemyState,
    rotation: f32,
    material_state: EnemyMaterialState,
}

impl Swordman {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Follow,
            material_state: EnemyMaterialState::None,
            rotation: 0.0,
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

        let mut dir_player = UnitVector3::new_normalize(player_pos - *obj_pos);
        *obj_pos += dir_player.into_inner() * delta_time;

        // let x_diff = player_pos.x - obj_pos.x;
        // let z_diff = player_pos.z - obj_pos.z;
        // let target_angle: f32 = x_diff.atan2(z_diff);
        // let delta_angle = target_angle - self.rotation;
        // if delta_angle.abs() > 0.01 {
        //     self.rotation += if delta_angle > 0.0 {
        //         -3.0 * delta_time
        //     } else {
        //         3.0 * delta_time
        //     };
        // }
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
        self.material_state = EnemyMaterialState::None;
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
