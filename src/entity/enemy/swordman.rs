use crate::animation::{InOutAnimation, InOutAnimationState};
use crate::entity::enemy::{HITTED_MATERIAL_DURATION, ROTATION_SPEED};
use crate::entity::HasMaterial;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::MaterialType;
use crate::scene::hit_and_dodge_scene::Score;
use crate::timer::Timer;
use nalgebra::{distance, Point, Rotation3, Unit, Vector3};
use rapier3d::prelude::Ray;

pub enum EnemyState {
    Attack(InOutAnimation),
    Chase,
}

pub enum EnemyMaterialState {
    None,
    Hitted(Timer),
}

pub struct Swordman {
    state: EnemyState,
    rotation_y: f32,
    dir: Vector3<f32>,
    material_state: EnemyMaterialState,
}

impl Swordman {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Chase,
            rotation_y: 0.0,
            material_state: EnemyMaterialState::None,
            dir: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        obj_pos: &mut Vector3<f32>,
        player_pos: &Vector3<f32>,
        physics: &GamePhysics,
        score: &mut Score,
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
        match &mut self.state {
            EnemyState::Attack(anim) => {
                let prev_state = anim.get_state();
                anim.update(delta_time);
                if let (InOutAnimationState::Foward(_), InOutAnimationState::Backward(_)) =
                    (prev_state, anim.get_state())
                {
                    let ray = Ray::new(
                        Point::from(*obj_pos) + current_dir.into_inner() * 0.6,
                        current_dir.into_inner(),
                    );
                    if let Some((x, _)) = physics.query_pipeline.cast_ray(
                        &physics.collider_set,
                        &ray,
                        0.5,
                        true,
                        physics.interaction_groups,
                        None,
                    ) {
                        let collider = physics.collider_set.get(x).unwrap();
                        if collider.user_data == u128::MAX {
                            score.hit_taken += 1;
                        }
                    }
                }
                if anim.get_value() == 0.0 {
                    self.state = EnemyState::Chase;
                }
            }
            EnemyState::Chase => {
                // This can be normalized. However, since the player
                // can't move fast, the enemy rotation velocity wouldn't
                // change much, so the normalized version of this vector is optional.
                let delta_dir = Unit::new_normalize(*current_dir - self.dir);

                let mut is_rotating = false;
                if delta_dir.x.abs() > 0.01 {
                    self.dir.x += delta_dir.x * delta_time * ROTATION_SPEED;
                    is_rotating |= true;
                }

                if delta_dir.z.abs() > 0.01 {
                    self.dir.z += delta_dir.z * delta_time * ROTATION_SPEED;
                    is_rotating |= true;
                }

                if is_rotating {
                    self.rotation_y = self.dir.x.atan2(self.dir.z);
                }
                let player_pos_xz = player_pos.xz();
                let current_pos_xz = obj_pos.xz();
                if distance(&Point::from(player_pos_xz), &Point::from(current_pos_xz)) <= 1.0 {
                    self.state = EnemyState::Attack(InOutAnimation::new_started(0.2, 0.2));
                } else {
                    let dir = Unit::new_normalize(player_pos_xz - current_pos_xz);
                    let next_pos = dir.into_inner() * 3.0 * delta_time;
                    obj_pos.x += next_pos.x;
                    obj_pos.z += next_pos.y;
                }
            }
        };
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        self.dir
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        Vector3::new(0.0, self.rotation_y, 0.0)
    }

    pub fn hitanim(&self) -> f32 {
        match self.state {
            EnemyState::Attack(ref anim) => -anim.get_value(),
            _ => 0.0,
        }
    }

    pub fn hit(&mut self) {
        self.material_state = EnemyMaterialState::Hitted(Timer::new(HITTED_MATERIAL_DURATION));
    }
}

impl HasMaterial for Swordman {
    fn get_material(&self) -> MaterialType {
        MaterialType::Black
        // match self.material_state {
        //     EnemyMaterialState::None => MaterialType::White,
        //     EnemyMaterialState::Hitted(_) => MaterialType::Red,
        // }
    }
}
