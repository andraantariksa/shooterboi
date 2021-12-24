use rapier3d::na::Vector3;

use crate::frustum::{Frustum, FrustumPlane};
use crate::util::clamp;

pub struct Camera {
    pub position: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    pub fov: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.6, 0.0),
            yaw: 270.0,
            pitch: 0.0,
            fov: 90.0f32.to_radians(),
            sensitivity: 0.5,
        }
    }

    pub fn get_direction_without_pitch(&self) -> nalgebra::Unit<nalgebra::Vector3<f32>> {
        nalgebra::Unit::new_normalize(nalgebra::Vector3::new(
            self.yaw.to_radians().cos(),
            0.0,
            self.yaw.to_radians().sin(),
        ))
    }

    pub fn move_direction(&mut self, offset: nalgebra::Vector2<f32>) {
        let offset_with_sensitivity = offset * self.sensitivity;
        self.yaw -= offset_with_sensitivity.x;
        self.pitch += offset_with_sensitivity.y;

        self.pitch = clamp(self.pitch, -89.0, 89.0);
    }

    pub fn get_direction(&self) -> nalgebra::Unit<nalgebra::Vector3<f32>> {
        nalgebra::Unit::new_normalize(nalgebra::Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ))
    }

    pub fn get_direction_right(&self) -> nalgebra::Unit<nalgebra::Vector3<f32>> {
        nalgebra::Unit::new_normalize(
            self.get_direction_without_pitch()
                .cross(&nalgebra::Vector3::new(0.0, 1.0, 0.0)),
        )
    }

    pub fn get_frustum(&self) -> Frustum {
        const Z_FAR: f32 = 150.0;
        const Z_NEAR: f32 = 0.1;
        let half_v_side = Z_FAR * (self.fov * 0.5).tan();
        let half_h_side = half_v_side * 1.5;
        let right = self.get_direction_right();
        let dir = self.get_direction();
        let front_times_far = Z_FAR * dir.into_inner();
        let up = nalgebra::Vector3::<f32>::new(0.0, 1.0, 0.0);

        Frustum {
            near: FrustumPlane::new(self.position + Z_NEAR * dir.into_inner(), dir),
            far: FrustumPlane::new(self.position + Z_FAR * dir.into_inner(), -dir),
            bottom: FrustumPlane::new(
                self.position,
                nalgebra::Unit::new_normalize(
                    (&(&front_times_far + up * half_v_side)).cross(&right),
                ),
            ),
            top: FrustumPlane::new(
                self.position,
                nalgebra::Unit::new_normalize(right.cross(&(&front_times_far - up * half_v_side))),
            ),
            left: FrustumPlane::new(
                self.position,
                nalgebra::Unit::new_normalize(
                    (&(&front_times_far - right.into_inner() * half_h_side)).cross(&up),
                ),
            ),
            right: FrustumPlane::new(
                self.position,
                nalgebra::Unit::new_normalize(
                    (&up).cross(&(&front_times_far + right.into_inner() * half_h_side)),
                ),
            ),
        }
    }
}
