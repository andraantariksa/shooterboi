use crate::nalgebra;

pub mod rendering_info;
pub(crate) mod vertex;

#[derive(Copy, Clone)]
pub enum ShapeType {
    None,
    Sphere,
    Box,
    Gun,
    CapsuleLine,
}

#[derive(Copy, Clone)]
pub enum RenderObjectType {
    Player,
    Ground,
    Enemy,
    Object,
}

#[derive(Copy, Clone)]
pub struct RenderQueueData {
    pub position: nalgebra::Vector3<f32>,
    _p1: [i32; 1],
    pub scale: nalgebra::Vector3<f32>,
    _p2: [i32; 1],
    pub rotation: nalgebra::Vector3<f32>,
    _p3: [i32; 1],
    pub color: nalgebra::Vector3<f32>,
    _p4: [i32; 1],
    pub shape_data1: nalgebra::Vector4<f32>,
    pub shape_data2: nalgebra::Vector4<f32>,
    pub shape_type: ShapeType,
    _p5: [i8; 3],
    pub object_type: RenderObjectType,
    _p6: [i8; 3],
}

impl RenderQueueData {
    pub fn new() -> Self {
        Self {
            color: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            scale: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            shape_type: ShapeType::None,
            object_type: RenderObjectType::Player,
            shape_data1: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
            shape_data2: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
            _p1: [0; 1],
            _p2: [0; 1],
            _p3: [0; 1],
            _p4: [0; 1],
            _p5: [0; 3],
            _p6: [0; 3],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::{RenderObjectType, RenderQueueData, ShapeType};

    #[test]
    fn render_queue_data_size() {
        assert_eq!(core::mem::size_of::<RenderQueueData>() % 16, 0);
    }

    #[test]
    fn render_object_type_size() {
        assert_eq!(core::mem::size_of::<RenderObjectType>(), 1);
    }

    #[test]
    fn shape_type_size() {
        assert_eq!(core::mem::size_of::<ShapeType>(), 1);
    }
}

pub struct Renderer {
    render_queue: [RenderQueueData; 100],
}
