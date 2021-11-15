use crate::camera::Camera;
use crate::renderer::rendering_info::RenderingInfo;
use crate::renderer::vertex::Vertex;
use crate::scene::ConrodHandle;
use crate::util::any_as_u8_slice;
use ambisonic::rodio::queue::queue;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

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
    pub fn new_none() -> Self {
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

struct GameSceneRenderer {
    main_render_pipeline: wgpu::RenderPipeline,
    screen_render_pipeline: wgpu::RenderPipeline,
    main_bind_group: wgpu::BindGroup,
    rendering_info_buffer: wgpu::Buffer,
    render_objects_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
}

impl GameSceneRenderer {
    fn new(
        surface_config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        rendering_info: &RenderingInfo,
        render_queue: &[RenderQueueData; 100],
    ) -> Self {
        let main_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Main fragment shader"),
            source: wgpu::include_spirv!("../shaders/main.frag.spv").source,
        });

        let screen_fragment_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Screen fragment shader"),
            source: wgpu::include_spirv!("../shaders/screen.frag.spv").source,
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
                        visibility: wgpu::ShaderStages::FRAGMENT,
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
            contents: any_as_u8_slice(render_queue),
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
                buffers: &[Vertex::desc()],
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
                    buffers: &[Vertex::desc()],
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

        Self {
            main_bind_group,
            screen_render_pipeline,
            main_render_pipeline,
            render_objects_buffer,
            rendering_info_buffer,
            vertex_buffer,
        }
    }

    fn render(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        surface_config: &SurfaceAndWindowConfig,
        conrod_handle: &mut ConrodHandle,
    ) {
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
    }
}

pub(crate) struct ConrodSceneRenderer {
    conrod_renderer: conrod_wgpu::Renderer,
    // pub(crate) ui: conrod_core::Ui,
    // app: conrod_example_shared::DemoApp,
    // image_map: conrod_core::image::Map<conrod_wgpu::Image>,
    // ids: conrod_example_shared::Ids,
}

impl ConrodSceneRenderer {
    fn new(
        surface_config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &mut wgpu::Queue,
    ) -> Self {
        // let mut ui = conrod_core::UiBuilder::new([
        //     surface_config.width as f64,
        //     surface_config.height as f64,
        // ])
        // .theme(conrod_example_shared::theme())
        // .build();
        // let ids = conrod_example_shared::Ids::new(ui.widget_id_generator());
        // ui.fonts.insert(
        //     conrod_core::text::Font::from_bytes(include_bytes!(
        //         "../../assets/fonts/NotoSans/NotoSans-Regular.ttf"
        //     ))
        //     .unwrap(),
        // );
        //
        // // Load the Rust logo from our assets folder to use as an example image.
        // let rgba_logo_image =
        //     image::load_from_memory(include_bytes!("../../assets/images/rust.png"))
        //         .expect("Couldn't load logo")
        //         .to_rgba8();
        //
        // // Create the GPU texture and upload the image data.
        // let (logo_w, logo_h) = rgba_logo_image.dimensions();
        // let logo_tex = create_logo_texture(&device, queue, rgba_logo_image);
        // let logo = conrod_wgpu::Image {
        //     texture: logo_tex,
        //     texture_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        //     width: logo_w,
        //     height: logo_h,
        // };
        // let mut image_map = conrod_core::image::Map::new();
        // let rust_logo = image_map.insert(logo);
        //
        // // Demonstration app state that we'll control with our conrod GUI.
        // let app = conrod_example_shared::DemoApp::new(rust_logo);
        Self {
            // app,
            // ui,
            // ids,
            // image_map,
            conrod_renderer: conrod_wgpu::Renderer::new(&device, 1, surface_config.format),
        }
    }

    fn render(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        surface_config: &SurfaceAndWindowConfig,
        conrod_handle: &mut ConrodHandle,
    ) {
        {
            let primitives = conrod_handle.get_ui().draw();
            if let Some(cmd) = self
                .conrod_renderer
                .fill(
                    &conrod_handle.get_image_map(),
                    [
                        0.0,
                        0.0,
                        surface_config.surface.width as f32,
                        surface_config.surface.height as f32,
                    ],
                    surface_config.window_scale_factor,
                    primitives,
                )
                .unwrap()
            {
                cmd.load_buffer_and_encode(device, encoder);
            }
        }
        let render = self
            .conrod_renderer
            .render(&device, &conrod_handle.get_image_map());
        let buffer_slice = render.vertex_buffer.slice(..);
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass gui"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
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

pub struct SurfaceAndWindowConfig {
    pub surface: wgpu::SurfaceConfiguration,
    pub window_scale_factor: f64,
}

pub struct Renderer {
    render_queue: [RenderQueueData; 100],
    rendering_info: RenderingInfo,
    pub(crate) camera: Camera,
    pub(crate) is_render_game: bool,
    pub(crate) is_render_gui: bool,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub(crate) surface_and_window_config: SurfaceAndWindowConfig,
    game_renderer: GameSceneRenderer,
    pub(crate) gui_renderer: ConrodSceneRenderer,
}

impl Renderer {
    pub(crate) async fn new(window: &Window) -> Self {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(if cfg!(target_arch = "wasm32") {
            wgpu::Backends::GL
        } else {
            wgpu::Backends::VULKAN
        });
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (mut device, mut queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .unwrap();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let mut render_queue = [RenderQueueData::new_none(); 100];
        render_queue[0].shape_type = ShapeType::Sphere;
        render_queue[0].shape_data1.x = 1.0;
        render_queue[0].position = nalgebra::Vector3::new(0.0, 0.0, -3.0);
        let rendering_info = RenderingInfo::new(window_size);

        Self {
            game_renderer: GameSceneRenderer::new(
                &surface_config,
                &device,
                &rendering_info,
                &render_queue,
            ),
            gui_renderer: ConrodSceneRenderer::new(&surface_config, &device, &mut queue),
            render_queue,
            rendering_info,
            is_render_gui: true,
            is_render_game: false,
            surface_and_window_config: SurfaceAndWindowConfig {
                surface: surface_config,
                window_scale_factor: window.scale_factor(),
            },
            surface,
            device,
            queue,
            camera: Camera::new(),
        }
    }

    pub(crate) fn resize(&mut self, new_size: &PhysicalSize<u32>, scale_factor: f64) {
        self.surface_and_window_config.surface.width = new_size.width;
        self.surface_and_window_config.surface.height = new_size.height;
        self.surface_and_window_config.window_scale_factor = scale_factor;
        self.rendering_info.resize(new_size);
        self.surface
            .configure(&self.device, &self.surface_and_window_config.surface);
    }

    pub(crate) fn render(
        &mut self,
        app_run_time: f32,
        conrod_handle: &mut ConrodHandle,
    ) -> Result<(), wgpu::SurfaceError> {
        if self.is_render_game {
            self.rendering_info.cam_pos = self.camera.position;
            self.rendering_info.cam_dir = self.camera.get_direction();
            self.rendering_info.reso_time.x = self.surface_and_window_config.surface.width as f32;
            self.rendering_info.reso_time.y = self.surface_and_window_config.surface.height as f32;
            self.rendering_info.reso_time.z = app_run_time;
            self.queue.write_buffer(
                &self.game_renderer.rendering_info_buffer,
                0,
                any_as_u8_slice(&self.rendering_info),
            );
        }

        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });
        {
            if self.is_render_game {
                self.game_renderer.render(
                    &view,
                    &mut encoder,
                    &self.device,
                    &self.surface_and_window_config,
                    conrod_handle,
                );
            }
            if self.is_render_gui {
                self.gui_renderer.render(
                    &view,
                    &mut encoder,
                    &self.device,
                    &self.surface_and_window_config,
                    conrod_handle,
                );
            }
        }
        self.queue.submit(core::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}

fn create_logo_texture(
    device: &wgpu::Device,
    queue: &mut wgpu::Queue,
    image: image::RgbaImage,
) -> wgpu::Texture {
    // Initialise the texture.
    let (width, height) = image.dimensions();
    let logo_tex_extent = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let logo_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("conrod_rust_logo_texture"),
        size: logo_tex_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    // Upload the pixel data.
    let data = &image.into_raw()[..];

    // Submit command for copying pixel data to the texture.
    let pixel_size_bytes = 4; // Rgba8, as above.
    let data_layout = wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: std::num::NonZeroU32::new(width * pixel_size_bytes),
        rows_per_image: std::num::NonZeroU32::new(height),
    };
    let texture_copy_view = wgpu::ImageCopyTexture {
        texture: &logo_tex,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    };
    let extent = wgpu::Extent3d {
        width: width,
        height: height,
        depth_or_array_layers: 1,
    };
    queue.write_texture(texture_copy_view, data, data_layout, extent);

    logo_tex
}
