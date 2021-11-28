use winit::dpi::PhysicalSize;
use winit::window::Window;

use conrod_renderer::ConrodSceneRenderer;
use game_renderer::GameSceneRenderer;

use crate::camera::{Camera, Frustum, ObjectBound};
use crate::gui::ConrodHandle;
use crate::renderer::crosshair::Crosshair;
use crate::renderer::rendering_info::RenderingInfo;
use crate::util::any_sized_as_u8_slice;

pub mod conrod_renderer;
pub mod crosshair;
pub mod game_renderer;
pub mod rendering_info;
pub mod vertex;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ShapeType {
    None = 0,
    Box = 1,
    Sphere = 2,
    Cylinder = 3,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderQueueData {
    pub position: nalgebra::Vector3<f32>,
    _p1: [i32; 1],
    pub scale: nalgebra::Vector3<f32>,
    _p2: [i32; 1],
    pub rotation: nalgebra::Vector3<f32>,
    _p3: [i32; 1],
    pub shape_data1: nalgebra::Vector4<f32>,
    pub shape_data2: nalgebra::Vector4<f32>,
    pub shape_type: ShapeType,
    _p4: [i32; 3],
}

impl RenderQueueData {
    pub fn new_none() -> Self {
        Self {
            position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            scale: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            shape_type: ShapeType::None,
            shape_data1: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
            shape_data2: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
            _p1: [0; 1],
            _p2: [0; 1],
            _p3: [0; 1],
            _p4: [0; 3],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::{RenderQueueData, ShapeType};

    #[test]
    fn render_queue_data_size() {
        assert_eq!(core::mem::size_of::<RenderQueueData>() % 16, 0);
    }

    #[test]
    fn shape_type_size() {
        assert_eq!(core::mem::size_of::<ShapeType>(), 4);
    }
}

pub struct SurfaceAndWindowConfig {
    pub surface: wgpu::SurfaceConfiguration,
    pub window_scale_factor: f64,
}

pub struct RenderObjects {
    render_objects: Vec<(RenderQueueData, ObjectBound)>,
    counter: usize,
    render_objects_static: Vec<(RenderQueueData, ObjectBound)>,
}

impl RenderObjects {
    pub fn new() -> Self {
        Self {
            counter: 0,
            render_objects: Vec::new(),
            render_objects_static: Vec::new(),
        }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut (RenderQueueData, ObjectBound) {
        self.render_objects.get_mut(index).unwrap()
    }

    pub fn get_mut_static(&mut self, index: usize) -> &mut (RenderQueueData, ObjectBound) {
        self.render_objects_static.get_mut(index).unwrap()
    }

    pub fn next(&mut self) -> &mut (RenderQueueData, ObjectBound) {
        let length = self.render_objects.len();
        self.render_objects
            .push((RenderQueueData::new_none(), ObjectBound::None));
        self.render_objects.get_mut(length).unwrap()
    }

    pub fn next_static(&mut self) -> &mut (RenderQueueData, ObjectBound) {
        let length = self.render_objects_static.len();
        self.render_objects_static
            .push((RenderQueueData::new_none(), ObjectBound::None));
        self.render_objects_static.get_mut(length).unwrap()
    }

    pub fn clear(&mut self) {
        self.render_objects.clear();
        self.render_objects_static.clear();
    }

    pub fn get_objects_and_active_len(
        &mut self,
        frustum: &Frustum,
    ) -> ([RenderQueueData; 50], usize) {
        let mut resulted_objects = [RenderQueueData::new_none(); 50];
        let mut index = 0;
        for (object, bound) in self.render_objects_static.iter() {
            if frustum.is_on_frustum(&object.position, bound) {
                resulted_objects[index] = *object;
                index += 1;
            }
        }
        for (object, bound) in self.render_objects.iter() {
            if frustum.is_on_frustum(&object.position, bound) {
                resulted_objects[index] = *object;
                index += 1;
            }
        }
        self.render_objects.clear();
        (resulted_objects, index)
    }
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
            let (objects, objects_len) = self
                .render_objects
                .get_objects_and_active_len(&self.camera.get_frustum());
            self.rendering_info.fov_shootanim.x = self.camera.fov;
            self.rendering_info.cam_pos = self.camera.position;
            self.rendering_info.cam_dir = *self.camera.get_direction();
            self.rendering_info.reso_time.x = self.surface_and_window_config.surface.width as f32;
            self.rendering_info.reso_time.y = self.surface_and_window_config.surface.height as f32;
            self.rendering_info.reso_time.z = app_run_time;
            self.rendering_info.queuecount_raymarchmaxstep_aostep.x = objects_len as u32;
            self.queue.write_buffer(
                &self.game_renderer.rendering_info_buffer,
                0,
                any_sized_as_u8_slice(&self.rendering_info),
            );
            self.queue.write_buffer(
                &self.game_renderer.render_objects_buffer,
                0,
                any_sized_as_u8_slice(&objects),
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
