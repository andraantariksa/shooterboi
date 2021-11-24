use crate::renderer::Renderer;

use conrod_core::image::Id as ImageId;
use conrod_core::text::font::Id as FontId;
use std::collections::HashMap;

pub struct ConrodHandle {
    ui: conrod_core::Ui,
    image_map: conrod_core::image::Map<conrod_wgpu::Image>,
    font_id_map: HashMap<&'static str, FontId>,
    image_id_map: HashMap<&'static str, ImageId>,
}

impl ConrodHandle {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut ui = conrod_core::UiBuilder::new([
            renderer.surface_and_window_config.surface.width as f64,
            renderer.surface_and_window_config.surface.height as f64,
        ])
        .theme(theme())
        .build();

        let mut font_id_map = HashMap::new();

        let noto_font = ui.fonts.insert(
            conrod_core::text::Font::from_bytes(include_bytes!(
                "../assets/fonts/NotoSans/NotoSans-Regular.ttf"
            ))
            .unwrap(),
        );
        let spy_font = ui.fonts.insert(
            conrod_core::text::Font::from_bytes(include_bytes!(
                "../assets/fonts/SpyAgency/spyagency3_2.ttf"
            ))
            .unwrap(),
        );
        let ropa_font = ui.fonts.insert(
            conrod_core::text::Font::from_bytes(include_bytes!(
                "../assets/fonts/RopaSans/RopaSans-Regular.ttf"
            ))
            .unwrap(),
        );

        font_id_map.insert("noto", noto_font);
        font_id_map.insert("spy", spy_font);
        font_id_map.insert("ropa", ropa_font);

        let mut image_map = conrod_core::image::Map::new();

        let mut image_id_map = HashMap::new();

        let title_image_rgba =
            image::load_from_memory(include_bytes!("../assets/images/title.png"))
                .unwrap()
                .to_rgba8();
        let (logo_w, texture_h) = title_image_rgba.dimensions();
        let title_texture = create_image_texture(
            &renderer.device,
            &mut renderer.queue,
            title_image_rgba,
            renderer.surface_and_window_config.surface.format,
        );
        let title_image = conrod_wgpu::Image {
            texture: title_texture,
            texture_format: renderer.surface_and_window_config.surface.format,
            width: logo_w,
            height: texture_h,
        };

        let title_image = image_map.insert(title_image);

        image_id_map.insert("title", title_image);

        Self {
            ui,
            image_map,
            image_id_map,
            font_id_map,
        }
    }

    pub fn get_font_id_map(&self) -> &HashMap<&'static str, FontId> {
        &self.font_id_map
    }

    pub fn get_image_id_map(&self) -> &HashMap<&'static str, ImageId> {
        &self.image_id_map
    }

    pub fn get_ui(&self) -> &conrod_core::Ui {
        &self.ui
    }

    pub fn get_ui_mut(&mut self) -> &mut conrod_core::Ui {
        &mut self.ui
    }

    pub fn get_image_map(&self) -> &conrod_core::image::Map<conrod_wgpu::Image> {
        &self.image_map
    }
}

pub fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

fn create_image_texture(
    device: &wgpu::Device,
    queue: &mut wgpu::Queue,
    image: image::RgbaImage,
    format: wgpu::TextureFormat,
) -> wgpu::Texture {
    let (width, height) = image.dimensions();
    let texture_extent = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Creating texture"),
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
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
        texture: &texture,
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

    texture
}
