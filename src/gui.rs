use crate::renderer::Renderer;
use crate::scene;

pub struct ConrodHandle {
    ui: conrod_core::Ui,
    image_map: conrod_core::image::Map<conrod_wgpu::Image>,
}

impl ConrodHandle {
    pub fn new(renderer: &Renderer) -> Self {
        let mut ui = conrod_core::UiBuilder::new([
            renderer.surface_and_window_config.surface.width as f64,
            renderer.surface_and_window_config.surface.height as f64,
        ])
        .theme(theme())
        .build();
        ui.fonts.insert(
            conrod_core::text::Font::from_bytes(include_bytes!(
                "../assets/fonts/NotoSans/NotoSans-Regular.ttf"
            ))
            .unwrap(),
        );
        let image_map = conrod_core::image::Map::new();
        Self { ui, image_map }
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
