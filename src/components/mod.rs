use crate::nalgebra::Vector3;

pub struct Target;

pub struct Transform {
    pub translation: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
