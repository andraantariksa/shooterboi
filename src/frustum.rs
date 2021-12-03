pub struct FrustumPlane {
    normal: nalgebra::Unit<nalgebra::Vector3<f32>>,
    distance: f32,
}

impl FrustumPlane {
    pub fn new(
        point: nalgebra::Vector3<f32>,
        normal: nalgebra::Unit<nalgebra::Vector3<f32>>,
    ) -> Self {
        let distance = normal.dot(&point);
        Self { normal, distance }
    }

    pub fn sd_to_plane(&self, coord: &nalgebra::Vector3<f32>) -> f32 {
        self.normal.dot(coord) - self.distance
    }
}

pub struct Frustum {
    pub top: FrustumPlane,
    pub bottom: FrustumPlane,
    pub left: FrustumPlane,
    pub right: FrustumPlane,
    pub near: FrustumPlane,
    pub far: FrustumPlane,
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
