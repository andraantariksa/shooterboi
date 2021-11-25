use nalgebra::{Vector2, Vector4};

pub struct CoordVertex {
    pub position: Vector2<f32>,
}

impl CoordVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vector2<f32>>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Coordinate
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const QUAD_VERTICES: [CoordVertex; 4] = [
    CoordVertex {
        position: Vector2::new(-1.0, -1.0),
    }, // Bottom left
    CoordVertex {
        position: Vector2::new(-1.0, 1.0),
    }, // Top left
    CoordVertex {
        position: Vector2::new(1.0, -1.0),
    }, // Bottom right
    CoordVertex {
        position: Vector2::new(1.0, 1.0),
    }, // Top right
];

pub struct CoordColorVertex {
    pub position: Vector2<f32>,
    pub color: Vector4<f32>,
}

impl CoordColorVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CoordColorVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Coordinate
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: core::mem::size_of::<Vector2<f32>>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
