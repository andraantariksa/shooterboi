use winit::dpi::PhysicalSize;
use winit::window::Window;

use conrod_renderer::ConrodSceneRenderer;
use game_renderer::GameSceneRenderer;
use render_objects::RenderObjects;

use crate::camera::Camera;

use crate::gui::ConrodHandle;
use crate::renderer::crosshair::Crosshair;
use crate::renderer::rendering_info::RenderingInfo;
use crate::util::{any_sized_as_u8_slice, any_slice_as_u8_slice};

pub mod conrod_renderer;
pub mod crosshair;
pub mod game_renderer;
pub mod render_objects;
pub mod rendering_info;
pub mod vertex;

pub struct SurfaceAndWindowConfig {
    pub surface: wgpu::SurfaceConfiguration,
    pub window_scale_factor: f64,
}

pub struct Renderer {
    pub render_objects: RenderObjects,
    pub rendering_info: RenderingInfo,
    pub camera: Camera,
    pub crosshair: Crosshair,
    pub is_render_game: bool,
    pub is_render_gui: bool,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_and_window_config: SurfaceAndWindowConfig,
    pub game_renderer: GameSceneRenderer,
    pub gui_renderer: ConrodSceneRenderer,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
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
        let (device, mut queue) = adapter
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

        let mut render_objects = RenderObjects::new();
        let rendering_info = RenderingInfo::new(window_size);

        let camera = Camera::new();
        let crosshair = Crosshair::new();

        Self {
            game_renderer: game_renderer::GameSceneRenderer::new(
                &surface_config,
                &device,
                &queue,
                &rendering_info,
                &mut render_objects,
                &camera,
                &crosshair,
            ),
            crosshair,
            gui_renderer: conrod_renderer::ConrodSceneRenderer::new(
                &surface_config,
                &device,
                &mut queue,
            ),
            render_objects,
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
            camera,
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>, scale_factor: f64) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_and_window_config.surface.width = new_size.width;
            self.surface_and_window_config.surface.height = new_size.height;
            self.surface_and_window_config.window_scale_factor = scale_factor;
            self.rendering_info.resize(new_size);
            self.surface
                .configure(&self.device, &self.surface_and_window_config.surface);
        }
    }

    pub fn render(
        &mut self,
        app_run_time: f32,
        conrod_handle: &mut ConrodHandle,
    ) -> Result<(), wgpu::SurfaceError> {
        if self.is_render_game {
            let objects = self
                .render_objects
                .get_objects_and_active_len(&self.camera.get_frustum());
            self.rendering_info.fov_shootanim.x = self.camera.fov;
            self.rendering_info.cam_pos = self.camera.position;
            self.rendering_info.cam_dir = *self.camera.get_direction();
            self.rendering_info.reso_time.x = self.surface_and_window_config.surface.width as f32;
            self.rendering_info.reso_time.y = self.surface_and_window_config.surface.height as f32;
            self.rendering_info.reso_time.z = app_run_time;
            self.rendering_info.queuecount_raymarchmaxstep_aostep.x = objects.len() as u32;
            self.queue.write_buffer(
                &self.game_renderer.rendering_info_buffer,
                0,
                any_sized_as_u8_slice(&self.rendering_info),
            );
            self.queue.write_buffer(
                &self.game_renderer.render_objects_buffer,
                0,
                any_slice_as_u8_slice(objects.as_slice()),
            );
        }

        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Texture view descriptor"),
            format: Some(self.surface_and_window_config.surface.format),
            ..Default::default()
        });

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
                    &self.crosshair,
                );
            }
            if self.is_render_gui {
                self.gui_renderer.render(
                    &view,
                    &mut encoder,
                    &self.device,
                    &self.surface_and_window_config,
                    conrod_handle,
                    self.is_render_game,
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
        width,
        height,
        depth_or_array_layers: 1,
    };
    queue.write_texture(texture_copy_view, data, data_layout, extent);

    logo_tex
}
