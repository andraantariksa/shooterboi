use crate::animation::{InOutAnimation, InOutAnimationState};

use crate::renderer::render_objects::MaterialType;

use crate::entity::enemy::{HITTED_MATERIAL_DURATION, ROTATION_SPEED};
use crate::entity::HasMaterial;

use crate::scene::{IN_SHOOT_ANIM_DURATION, OUT_SHOOT_ANIM_DURATION};
use crate::timer::Timer;
use nalgebra::{Rotation3, Unit, Vector2, Vector3};
use rand::distributions::Uniform;
use rand::prelude::*;

pub struct Bullet;

impl HasMaterial for Bullet {
    fn get_material(&self) -> MaterialType {
        MaterialType::Black
    }
}

pub const BULLET_SPEED: f32 = 50.0;
pub const BULLET_RAD: f32 = 0.2;

pub enum ShootState {
    Shoot(InOutAnimation),
    Focus(Timer),
    Idle(Timer),
}

pub enum GunmanOp {
    None,
    Shoot {
        pos: Vector3<f32>,
        dir: Vector3<f32>,
    },
}

pub enum EnemyMaterialState {
    None,
    Hitted(Timer),
}

pub struct Gunman {
    shoot_state: ShootState,
    rotation_y: f32,
    dir: Vector3<f32>,
    material_state: EnemyMaterialState,
    next_dest: Vector2<f32>,
}

impl Gunman {
    pub fn new(rng: &mut SmallRng) -> Self {
        Self {
            shoot_state: ShootState::Idle(Timer::new(0.5)),
            rotation_y: 0.0,
            dir: Vector3::new(0.0, 1.0, 0.0),
            material_state: EnemyMaterialState::None,
            next_dest: Vector2::new(
                rng.sample(Uniform::new(-13.0, 13.0)),
                rng.sample(Uniform::new(-13.0, 13.0)),
            ),
        }
    }

    pub fn update(
        &mut self,
        rng: &mut SmallRng,
        delta_time: f32,
        obj_pos: &mut Vector3<f32>,
        player_pos: &Vector3<f32>,
    ) -> GunmanOp {
        match self.material_state {
            EnemyMaterialState::Hitted(ref mut timer) => {
                timer.update(delta_time);
                if timer.is_finished() {
                    self.material_state = EnemyMaterialState::None;
                }
            }
            _ => {}
        };

        let current_xz_pos = obj_pos.xz();
        if current_xz_pos.relative_eq(&self.next_dest, f32::EPSILON, 0.5) {
            self.next_dest = Vector2::new(
                rng.sample(Uniform::new(-9.0, 9.0)),
                rng.sample(Uniform::new(-9.0, 9.0)),
            );
        } else {
            let dir = Unit::new_normalize(self.next_dest - current_xz_pos);
            let next_pos = dir.into_inner() * 3.0 * delta_time;
            obj_pos.x += next_pos.x;
            obj_pos.z += next_pos.y;
        }

        let mut op = GunmanOp::None;

        match self.shoot_state {
            ShootState::Idle(ref mut timer) => {
                timer.update(delta_time);

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
                    self.rotation_y = self.dir.x.atan2(self.dir.z);
                } else if timer.is_finished() {
                    self.shoot_state = ShootState::Focus(Timer::new(0.5));
                }
            }
            ShootState::Focus(ref mut timer) => {
                timer.update(delta_time);
                if timer.is_finished() {
                    op = GunmanOp::Shoot {
                        dir: self.dir,
                        pos: *obj_pos,
                    };
                    self.shoot_state = ShootState::Shoot(InOutAnimation::new_started(
                        IN_SHOOT_ANIM_DURATION,
                        OUT_SHOOT_ANIM_DURATION,
                    ));
                }
            }
            ShootState::Shoot(ref mut anim) => {
                anim.update(delta_time);
                match anim.get_state() {
                    InOutAnimationState::Stopped => {
                        self.shoot_state = ShootState::Idle(Timer::new(0.5));
                    }
                    _ => {}
                };
            }
        };

        op
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        self.dir
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        Vector3::new(0.0, self.rotation_y, 0.0)
    }

    pub fn shootanim(&self) -> f32 {
        match self.shoot_state {
            ShootState::Shoot(ref anim) => -anim.get_value() * 1.5,
            _ => 0.0,
        }
    }

    pub fn hit(&mut self) {
        self.material_state = EnemyMaterialState::Hitted(Timer::new(HITTED_MATERIAL_DURATION));
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
