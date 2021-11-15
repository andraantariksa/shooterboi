use crate::nalgebra;
use conrod_core::image::Map;
use conrod_core::Ui;
use conrod_wgpu::{Image, Renderer};
use core::iter;

use wgpu::util::{DeviceExt, StagingBelt};
use wgpu::{BindGroupLayoutDescriptor, ShaderStages};
use winit::event::WindowEvent;
use winit::window::Window;

use crate::renderer::rendering_info::RenderingInfo;
use crate::renderer::vertex::Vertex;
use crate::renderer::{RenderQueueData, ShapeType};
use crate::util::any_as_u8_slice;

const QUAD_VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, -1.0, 0.0],
    }, // Bottom left
    Vertex {
        position: [-1.0, 1.0, 0.0],
    }, // Top left
    Vertex {
        position: [1.0, -1.0, 0.0],
    }, // Bottom right
    Vertex {
        position: [1.0, 1.0, 0.0],
    }, // Top right
];

pub struct State {
    surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub(crate) window_size: winit::dpi::PhysicalSize<u32>,
    main_render_pipeline: wgpu::RenderPipeline,
    screen_render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    main_bind_group: wgpu::BindGroup,
    rendering_info_buffer: wgpu::Buffer,
    render_objects: [RenderQueueData; 100],
    render_objects_buffer: wgpu::Buffer,
    pub(crate) surface_preferred_format: wgpu::TextureFormat,
    // egui_rpass: egui_wgpu_backend::RenderPass,
    conrod_renderer: conrod_wgpu::Renderer,
}

impl State {
    pub(crate) async fn new(window: &Window, rendering_info: &RenderingInfo) -> Self {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(if cfg!(target_arch = "wasm32") {
            wgpu::Backends::all()
        } else {
            wgpu::Backends::VULKAN
        });
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_preferred_format = surface.get_preferred_format(&adapter).unwrap();
        // let mut egui_rpass = egui_wgpu_backend::RenderPass::new(&device, surface_format, 1);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let main_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Main fragment shader"),
            source: wgpu::include_spirv!("./shaders/main.frag.spv").source,
        });

        let screen_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Screen fragment shader"),
            source: wgpu::include_spirv!("./shaders/screen.frag.spv").source,
        });

        let vertex_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Vertex shader"),
            source: wgpu::include_spirv!("./shaders/main.vert.spv").source,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad vertex buffer"),
            contents: any_as_u8_slice(&QUAD_VERTICES), // bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let rendering_info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rendering info buffer"),
            contents: any_as_u8_slice(rendering_info),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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
                        visibility: ShaderStages::FRAGMENT,
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
                        visibility: ShaderStages::FRAGMENT,
                    },
                ],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&main_bindgroup_layout],
                push_constant_ranges: &[],
            });

        let mut render_objects = [RenderQueueData::new_none(); 100];
        render_objects[0].shape_type = ShapeType::Sphere;
        render_objects[0].shape_data1.x = 1.0;
        render_objects[0].position = nalgebra::Vector3::new(0.0, 0.0, -3.0);

        let render_objects_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rendering objects"),
            contents: any_as_u8_slice(&render_objects),
            usage: if cfg!(target_arch = "wasm32") {
                wgpu::BufferUsages::UNIFORM
            } else {
                wgpu::BufferUsages::STORAGE
            } | wgpu::BufferUsages::COPY_DST,
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
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &main_fragment_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
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
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &screen_fragment_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: config.format,
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

        let conrod_renderer = conrod_wgpu::Renderer::new(&device, 1, surface_preferred_format);

        Self {
            surface,
            device,
            queue,
            config,
            window_size,
            main_render_pipeline,
            screen_render_pipeline,
            vertex_buffer,
            main_bind_group,
            rendering_info_buffer,
            render_objects,
            render_objects_buffer,
            surface_preferred_format, // egui_rpass,
            conrod_renderer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.window_size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub(crate) fn update(&mut self) {}

    pub(crate) fn render(
        &mut self,
        rendering_info: &RenderingInfo,
        window: &mut Window,
        ui: &Ui,
        image_map: &Map<Image>,
    ) -> Result<(), wgpu::SurfaceError> {
        self.queue.write_buffer(
            &self.rendering_info_buffer,
            0,
            any_as_u8_slice(rendering_info),
        );

        let frame = self.surface.get_current_frame()?;
        let view = frame
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });
        {
            let primitives = ui.draw();
            if let Some(cmd) = self
                .conrod_renderer
                .fill(
                    &image_map,
                    [
                        0.0,
                        0.0,
                        self.window_size.width as f32,
                        self.window_size.height as f32,
                    ],
                    window.scale_factor(),
                    primitives,
                )
                .unwrap()
            {
                cmd.load_buffer_and_encode(&self.device, &mut encoder);
            }
        }
        let render = self.conrod_renderer.render(&self.device, &image_map);
        let buffer_slice = render.vertex_buffer.slice(..);
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
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

            {
                let slot = 0;
                render_pass.set_vertex_buffer(slot, buffer_slice);
                for cmd in render.commands {
                    match cmd {
                        conrod_wgpu::RenderPassCommand::SetPipeline { pipeline } => {
                            render_pass.set_pipeline(pipeline);
                        }
                        conrod_wgpu::RenderPassCommand::SetBindGroup { bind_group } => {
                            render_pass.set_bind_group(0, bind_group, &[]);
                        }
                        conrod_wgpu::RenderPassCommand::SetScissor {
                            top_left: [x, y],
                            dimensions: [w, h],
                        } => {
                            render_pass.set_scissor_rect(0, 0, w, h);
                        }
                        conrod_wgpu::RenderPassCommand::Draw { vertex_range } => {
                            render_pass.draw(vertex_range, 0..1);
                        }
                    }
                }
            }
        }

        // self.egui_rpass
        //     .update_texture(&self.device, &self.queue, &platform.context().texture());
        // self.egui_rpass.update_user_textures(&device, &self.queue);
        // self.egui_rpass
        //     .update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descriptor);
        // self.egui_rpass
        //     .execute(
        //         &mut encoder,
        //         &output_view,
        //         &paint_jobs,
        //         &screen_descriptor,
        //         Some(wgpu::Color::BLACK),
        //     )
        //     .unwrap();

        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::vertex::Vertex;
    use crate::state::QUAD_VERTICES;

    #[test]
    fn vertices_size() {
        assert_eq!(
            core::mem::size_of::<Vertex>() * QUAD_VERTICES.len(),
            core::mem::size_of::<f32>() * 3 * 4
        );
    }
}
