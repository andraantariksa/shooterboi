use wgpu::util::DeviceExt;
use wgpu::{Device, SurfaceConfiguration};

use crate::camera::Camera;
use crate::gui::ConrodHandle;
use crate::renderer::crosshair::Crosshair;
use crate::renderer::rendering_info::RenderingInfo;
use crate::renderer::vertex::{CoordColorVertex, CoordVertex, QUAD_VERTICES};
use crate::renderer::{RenderObjects, SurfaceAndWindowConfig};
use crate::util::{any_sized_as_u8_slice, any_slice_as_u8_slice};

pub struct GameSceneRenderer {
    pub main_render_pipeline: wgpu::RenderPipeline,
    pub screen_render_pipeline: wgpu::RenderPipeline,
    pub crosshair_render_pipeline: wgpu::RenderPipeline,
    pub main_bind_group: wgpu::BindGroup,
    pub rendering_info_buffer: wgpu::Buffer,
    pub render_objects_buffer: wgpu::Buffer,
    pub quad_vertex_buffer: wgpu::Buffer,
    pub crosshair_vertex_buffer: wgpu::Buffer,
}

impl GameSceneRenderer {
    pub fn new(
        surface_config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rendering_info: &RenderingInfo,
        render_objects: &mut RenderObjects,
        camera: &Camera,
        crosshair: &Crosshair,
    ) -> Self {
        let main_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Main fragment shader"),
            source: wgpu::include_spirv!("../shaders/main.frag.spv").source,
        });

        let screen_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Screen fragment shader"),
            source: wgpu::include_spirv!("../shaders/screen.frag.spv").source,
        });

        let crosshair_vertex_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Crosshair vertex shader"),
            source: wgpu::include_spirv!("../shaders/crosshair.vert.spv").source,
        });

        let crosshair_fragment_shader =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Crosshair fragment shader"),
                source: wgpu::include_spirv!("../shaders/crosshair.frag.spv").source,
            });

        let main_vertex_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Main vertex shader"),
            source: wgpu::include_spirv!("../shaders/main.vert.spv").source,
        });

        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad vertex buffer"),
            contents: any_sized_as_u8_slice(&QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let crosshair_vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Crosshair vertex buffer"),
                contents: any_slice_as_u8_slice(crosshair.get_vertices().as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        let render_objects_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rendering objects"),
            contents: any_sized_as_u8_slice(
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
            contents: any_sized_as_u8_slice(rendering_info),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::VERTEX,
        });

        let image = image::load_from_memory(include_bytes!("../../assets/images/checker.png"))
            .unwrap()
            .to_rgba8();
        let ground_texture = device.create_texture_with_data(
            &queue,
            &wgpu::TextureDescriptor {
                label: Some("Ground texture"),
                size: wgpu::Extent3d {
                    width: image.width(),
                    height: image.height(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
            },
            &image.into_raw()[..],
        );
        let ground_texture_view = ground_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Ground texture view"),
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
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
                    wgpu::BindGroupLayoutEntry {
                        count: None,
                        binding: 2,
                        ty: wgpu::BindingType::Sampler {
                            filtering: true,
                            comparison: false,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                    wgpu::BindGroupLayoutEntry {
                        count: None,
                        binding: 3,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                ],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[&main_bindgroup_layout],
                push_constant_ranges: &[],
            });

        let main_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Main bind group"),
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
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&ground_texture_view),
                },
            ],
        });

        let main_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Main render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &main_vertex_shader,
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
                label: Some("Screen render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &main_vertex_shader,
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
                label: Some("Crosshair render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &crosshair_vertex_shader,
                    entry_point: "main",
                    buffers: &[CoordColorVertex::desc()],
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
                    topology: wgpu::PrimitiveTopology::TriangleList,
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
            quad_vertex_buffer,
            crosshair_vertex_buffer,
        }
    }

    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        _device: &wgpu::Device,
        _surface_config: &SurfaceAndWindowConfig,
        _conrod_handle: &mut ConrodHandle,
        crosshair: &Crosshair,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Game render pass"),
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

        render_pass.set_bind_group(0, &self.main_bind_group, &[]);
        render_pass.set_pipeline(&self.main_render_pipeline);
        render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        render_pass.draw(0..4, 0..1);

        render_pass.set_pipeline(&self.screen_render_pipeline);
        render_pass.draw(0..4, 0..1);

        render_pass.set_pipeline(&self.crosshair_render_pipeline);
        render_pass.set_vertex_buffer(0, self.crosshair_vertex_buffer.slice(..));
        render_pass.draw(0..crosshair.vertices_len(), 0..1);
    }
}
