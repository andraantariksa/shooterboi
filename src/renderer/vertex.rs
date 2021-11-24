pub struct CoordVertex {
    pub position: [f32; 2],
}

impl CoordVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CoordVertex>() as wgpu::BufferAddress,
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
        position: [-1.0, -1.0],
    }, // Bottom left
    CoordVertex {
        position: [-1.0, 1.0],
    }, // Top left
    CoordVertex {
        position: [1.0, -1.0],
    }, // Bottom right
    CoordVertex {
        position: [1.0, 1.0],
    }, // Top right
];
