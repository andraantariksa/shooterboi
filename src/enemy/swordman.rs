use crate::enemy::{HasMaterial, HITTED_MATERIAL_DURATION};
use crate::renderer::render_objects::MaterialType;
use nalgebra::Vector3;

pub enum EnemyState {
    Attack,
    ChasePlayer,
}

pub enum EnemyMaterialState {
    None,
    Shooted(f32),
}

pub struct Swordman {
    state: EnemyState,
    material_state: EnemyMaterialState,
}

impl Swordman {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Attack,
            material_state: EnemyMaterialState::None,
        }
    }

    pub fn update(&mut self, delta_time: f32, player_pos: Vector3<f32>) {
        match self.material_state {
            EnemyMaterialState::Shooted(ref mut duration) => {
                *duration -= delta_time;
                if *duration <= 0.0f32 {
                    self.material_state = EnemyMaterialState::None
                }
            }
            _ => {}
        };
        // match self.state {
        //     EnemyState::ChasePlayer => {}
        // };
    }

    pub fn hit(&mut self) {
        self.material_state = EnemyMaterialState::None;
    }
}

impl HasMaterial for Swordman {
    fn get_material(&self) -> MaterialType {
        MaterialType::White
    }
}
