use winit::dpi::PhysicalSize;

#[derive(Debug, Clone)]
#[repr(u32)]
pub enum BackgroundType {
    None = 0,
    Forest = 1,
    City = 2,
}

#[derive(Debug, Clone)]
pub struct RenderingInfo {
    pub reso_time: nalgebra::Vector3<f32>,
    _p1: [i32; 1],
    pub cam_pos: nalgebra::Vector3<f32>,
    _p2: [i32; 1],
    pub cam_dir: nalgebra::Vector3<f32>,
    _p3: [i32; 1],
    pub fov_shootanim: nalgebra::Vector2<f32>,
    _p4: [i32; 2],
    pub queuecount_raymarchmaxstep_aostep: nalgebra::Vector3<u32>,
    pub background_type: BackgroundType,
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
            fov_shootanim: nalgebra::Vector2::new(90.0f32.to_radians(), 0.0),
            queuecount_raymarchmaxstep_aostep: nalgebra::Vector3::new(0, 50, 3),
            background_type: BackgroundType::None,
            _p2: [0; 1],
            _p3: [0; 1],
            _p4: [0; 2],
            _p1: [0; 1],
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.reso_time.x = new_size.width as f32;
        self.reso_time.y = new_size.height as f32;
    }
}
