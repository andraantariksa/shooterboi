use crate::renderer::vertex::{CoordColorVertex, CoordVertex};
use crate::util::any_slice_as_u8_slice;
use nalgebra::{Rotation2, Vector2, Vector3, Vector4};

pub struct Crosshair {
    pub color: Vector3<f32>,
    pub inner_line_enabled: bool,
    pub inner_line_thickness: f32,
    pub inner_line_offset: f32,
    pub inner_line_length: f32,
    // pub inner_line_opacity: f32,
    pub outer_line_enabled: bool,
    pub outer_line_thickness: f32,
    pub outer_line_offset: f32,
    pub outer_line_length: f32,
    // pub outer_line_opacity: f32,
    pub center_dot_enabled: bool,
    // pub center_dot_opacity: f32,
    pub center_dot_thickness: f32,
}

impl Crosshair {
    pub fn new() -> Self {
        Self {
            color: Vector3::new(0.0, 1.0, 0.0),

            inner_line_thickness: 5.0,
            inner_line_offset: 6.0,
            inner_line_length: 20.0,
            // inner_line_opacity: 0.777,
            outer_line_thickness: 4.0,
            outer_line_offset: 29.0,
            outer_line_length: 6.0,
            // outer_line_opacity: 1.0,

            // center_dot_opacity: 1.0,
            center_dot_thickness: 3.0,

            center_dot_enabled: true,
            inner_line_enabled: true,
            outer_line_enabled: true,
        }
    }

    pub fn get_vertices(&self) -> Vec<CoordColorVertex> {
        let deg90 = Rotation2::new(std::f32::consts::FRAC_PI_2);
        let deg180 = Rotation2::new(std::f32::consts::PI);
        let deg270 = Rotation2::new(std::f32::consts::FRAC_PI_2 * 3.0);
        let all_rot = [deg90, deg180, deg270];

        let mut resulting_vertices = Vec::<CoordColorVertex>::new();

        if self.center_dot_enabled {
            let color = Vector4::new(self.color.x, self.color.y, self.color.z, 1.0);
            let dot = [
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        -self.center_dot_thickness,
                        -self.center_dot_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        -self.center_dot_thickness,
                        self.center_dot_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.center_dot_thickness,
                        -self.center_dot_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.center_dot_thickness,
                        -self.center_dot_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.center_dot_thickness,
                        self.center_dot_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        -self.center_dot_thickness,
                        self.center_dot_thickness,
                    ),
                    color,
                },
            ];
            resulting_vertices.extend(dot);
        }

        if self.inner_line_enabled {
            let color = Vector4::new(self.color.x, self.color.y, self.color.z, 1.0);
            let inner_line = [
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.inner_line_offset,
                        -self.inner_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.inner_line_offset,
                        self.inner_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.inner_line_offset + self.inner_line_length,
                        -self.inner_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.inner_line_offset + self.inner_line_length,
                        -self.inner_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.inner_line_offset + self.inner_line_length,
                        self.inner_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.inner_line_offset,
                        self.inner_line_thickness,
                    ),
                    color,
                },
            ];
            for rot in all_rot.iter() {
                let resulting_inner_line = inner_line
                    .iter()
                    .map(|v| CoordColorVertex {
                        position: rot * v.position,
                        color: v.color,
                    })
                    .collect::<Vec<CoordColorVertex>>();
                resulting_vertices.extend(resulting_inner_line);
            }
            resulting_vertices.extend(inner_line);
        }

        if self.outer_line_enabled {
            let color = Vector4::new(self.color.x, self.color.y, self.color.z, 1.0);
            let outer_line = [
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.outer_line_offset,
                        -self.outer_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.outer_line_offset,
                        self.outer_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.outer_line_offset + self.outer_line_length,
                        -self.outer_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.outer_line_offset + self.outer_line_length,
                        -self.outer_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.outer_line_offset + self.outer_line_length,
                        self.outer_line_thickness,
                    ),
                    color,
                },
                CoordColorVertex {
                    position: Vector2::<f32>::new(
                        self.outer_line_offset,
                        self.outer_line_thickness,
                    ),
                    color,
                },
            ];
            for rot in all_rot.iter() {
                let resulting_outer_line = outer_line
                    .iter()
                    .map(|v| CoordColorVertex {
                        position: rot * v.position,
                        color: v.color,
                    })
                    .collect::<Vec<CoordColorVertex>>();
                resulting_vertices.extend(resulting_outer_line);
            }
            resulting_vertices.extend(outer_line);
        }
        resulting_vertices
    }

    pub fn update_vertices(
        &self,
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        offset: wgpu::BufferAddress,
    ) {
        let vertices = self.get_vertices();
        queue.write_buffer(buffer, offset, any_slice_as_u8_slice(vertices.as_slice()));
    }

    pub fn vertices_len(&self) -> u32 {
        0 + if self.outer_line_enabled { 6 * 4 } else { 0 }
            + if self.inner_line_enabled { 6 * 4 } else { 0 }
            + if self.center_dot_enabled { 6 } else { 0 }
    }
}
