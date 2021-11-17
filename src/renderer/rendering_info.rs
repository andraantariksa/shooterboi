use winit::dpi::PhysicalSize;

#[derive(Debug, Clone)]
pub struct RenderingInfo {
    pub(crate) reso_time: nalgebra::Vector3<f32>,
    _p1: [i32; 1],
    pub(crate) cam_pos: nalgebra::Vector3<f32>,
    _p2: [i32; 1],
    pub(crate) cam_dir: nalgebra::Vector3<f32>,
    pub(crate) fov: f32,
    pub(crate) queue_count: u32,
    _p3: [i32; 3],
}

impl RenderingInfo {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            reso_time: nalgebra::Vector3::new(
                window_size.width as f32,
                window_size.height as f32,
                0.0,
            ),
            cam_pos: nalgebra::Vector3::new(0.0, 1.0, 0.0),
            cam_dir: nalgebra::Vector3::new(0.0, 0.0, -1.0),
            fov: 90.0f32.to_radians(),
            queue_count: 0,
            _p1: [0; 1],
            _p2: [0; 1],
            _p3: [0; 3],
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.reso_time.x = new_size.width as f32;
        self.reso_time.y = new_size.height as f32;
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::rendering_info::RenderingInfo;

    #[test]
    fn rendering_info_size() {
        assert_eq!(core::mem::size_of::<RenderingInfo>() % 16, 0);
    }
}
