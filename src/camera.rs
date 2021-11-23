use crate::util::clamp;

use rapier3d::na::Vector3;

pub struct FrustumPlane {
    normal: nalgebra::Unit<nalgebra::Vector3<f32>>,
    distance: f32,
}

impl FrustumPlane {
    fn new(point: nalgebra::Vector3<f32>, normal: nalgebra::Unit<nalgebra::Vector3<f32>>) -> Self {
        let distance = normal.dot(&point);
        Self { normal, distance }
    }

    fn sd_to_plane(&self, coord: &nalgebra::Vector3<f32>) -> f32 {
        self.normal.dot(coord) - self.distance
    }
}

pub struct Frustum {
    top: FrustumPlane,
    bottom: FrustumPlane,
    left: FrustumPlane,
    right: FrustumPlane,
    near: FrustumPlane,
    far: FrustumPlane,
}

impl Frustum {
    pub fn is_on_frustum(&self, position: &nalgebra::Vector3<f32>, bound: &ObjectBound) -> bool {
        match bound {
            ObjectBound::Sphere(r) => {
                self.top.sd_to_plane(position) > -*r
                    && self.bottom.sd_to_plane(position) > -*r
                    && self.left.sd_to_plane(position) > -*r
                    && self.right.sd_to_plane(position) > -*r
                    && self.near.sd_to_plane(position) > -*r
                    && self.far.sd_to_plane(position) > -*r
            }
            ObjectBound::None => true,
        }
    }
}

pub enum ObjectBound {
    Sphere(f32),
    None,
}

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
        const Z_FAR: f32 = 100.0;
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
