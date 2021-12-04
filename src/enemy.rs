use crate::renderer::MaterialType;
use nalgebra::Vector3;

pub enum EnemyState {
    Patrol,
    ChasePlayer,
}

pub enum EnemyMaterialState {
    None,
    Shooted(f32),
}

pub struct Enemy {
    health: f32,
    state: EnemyState,
    material_state: EnemyMaterialState,
}

const SHOOTED_MATERIAL_DURATION: f32 = 0.2;

impl Enemy {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            state: EnemyState::Patrol,
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
        match self.state {
            EnemyState::Patrol => {}
            EnemyState::ChasePlayer => {}
        };
    }

    pub fn shooted(&mut self) {
        self.health -= 30.0;
        self.material_state = EnemyMaterialState::Shooted(SHOOTED_MATERIAL_DURATION);
    }

    pub fn is_died(&self) -> bool {
        self.health <= 0.0
    }

    pub fn get_material(&self) -> MaterialType {
        MaterialType::Green
    }
}
