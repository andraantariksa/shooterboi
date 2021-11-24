use wgpu::util::DeviceExt;

use crate::camera::Camera;
use crate::gui::ConrodHandle;
use crate::renderer::rendering_info::RenderingInfo;
use crate::renderer::vertex::{CoordVertex, QUAD_VERTICES};
use crate::renderer::{RenderObjects, SurfaceAndWindowConfig};
use crate::util::any_as_u8_slice;

pub struct GameSceneRenderer {
    pub main_render_pipeline: wgpu::RenderPipeline,
    pub screen_render_pipeline: wgpu::RenderPipeline,
    pub crosshair_render_pipeline: wgpu::RenderPipeline,
    pub main_bind_group: wgpu::BindGroup,
    pub rendering_info_buffer: wgpu::Buffer,
    pub render_objects_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
}

impl GameSceneRenderer {
    pub fn new(
        surface_config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        rendering_info: &RenderingInfo,
        render_objects: &mut RenderObjects,
        camera: &Camera,
    ) -> Self {
        let main_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Main fragment shader"),
            source: wgpu::include_spirv!("../shaders/main.frag.spv").source,
        });

        let screen_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Screen fragment shader"),
            source: wgpu::include_spirv!("../shaders/screen.frag.spv").source,
        });

        let crosshair_fragment_shader =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Crosshair fragment shader"),
                source: wgpu::include_spirv!("../shaders/crosshair.frag.spv").source,
            });

        let vertex_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Vertex shader"),
            source: wgpu::include_spirv!("../shaders/main.vert.spv").source,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad vertex buffer"),
            contents: any_as_u8_slice(&QUAD_VERTICES), // bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let main_bindgroup_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind group layout descriptor main"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        count: None,
                        binding: 0,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                    },
                    wgpu::BindGroupLayoutEntry {
                        count: None,
                        binding: 1,
                        ty: wgpu::BindingType::Buffer {
                            ty: if cfg!(target_arch = "wasm32") {
                                wgpu::BufferBindingType::Uniform
                            } else {
                                wgpu::BufferBindingType::Storage { read_only: true }
                            },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                ],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&main_bindgroup_layout],
                push_constant_ranges: &[],
            });

        let render_objects_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rendering objects"),
            contents: any_as_u8_slice(
                &render_objects
                    .get_objects_and_active_len(&camera.get_frustum())
                    .0,
            ),
            usage: if cfg!(target_arch = "wasm32") {
                wgpu::BufferUsages::UNIFORM
            } else {
                wgpu::BufferUsages::STORAGE
            } | wgpu::BufferUsages::COPY_DST,
        });

        let rendering_info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rendering info buffer"),
            contents: any_as_u8_slice(rendering_info),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let main_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("main bind group"),
            layout: &main_bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: rendering_info_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: render_objects_buffer.as_entire_binding(),
                },
            ],
        });

        let main_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "main",
                buffers: &[CoordVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &main_fragment_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let screen_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "main",
                    buffers: &[CoordVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &screen_fragment_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });

        let crosshair_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "main",
                    buffers: &[CoordVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &crosshair_fragment_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });

        Self {
            main_bind_group,
            main_render_pipeline,
            screen_render_pipeline,
            crosshair_render_pipeline,
            render_objects_buffer,
            rendering_info_buffer,
            vertex_buffer,
        }
    }

    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        _device: &wgpu::Device,
        _surface_config: &SurfaceAndWindowConfig,
        _conrod_handle: &mut ConrodHandle,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.main_render_pipeline);
        render_pass.set_bind_group(0, &self.main_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..4, 0..1);

        render_pass.set_pipeline(&self.screen_render_pipeline);
        render_pass.draw(0..4, 0..1);
    }
}
