use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget};
use hecs::World;
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;

pub enum SceneOp {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Replace(Box<dyn Scene>),
}

pub trait Scene {
    fn update(
        &mut self,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        control_flow: &mut ControlFlow,
    ) -> SceneOp;
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

widget_ids! {
    pub struct MainMenuSceneIds {
        // The main canvas
        canvas,
        quit_button,
        guide_button,
        settings_button,
        start_classic_mode_button
    }
}

pub struct MainMenuScene {
    ids: MainMenuSceneIds,
}

impl MainMenuScene {
    pub fn new(renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;

        Self {
            ids: MainMenuSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
        }
    }
}

impl Scene for MainMenuScene {
    fn update(
        &mut self,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        control_flow: &mut ControlFlow,
    ) -> SceneOp {
        let mut scene_op = SceneOp::None;

        const MARGIN: conrod_core::Scalar = 30.0;

        let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

        conrod_core::widget::Canvas::new()
            .pad(MARGIN)
            .color(Color::Rgba(1.0, 1.0, 1.0, 0.0))
            .set(self.ids.canvas, &mut ui_cell);

        for _press in conrod_core::widget::Button::new()
            .label("Classic Mode")
            .wh(conrod_core::Dimensions::new(100.0, 100.0))
            .mid_left_with_margin_on(self.ids.canvas, MARGIN)
            .set(self.ids.start_classic_mode_button, &mut ui_cell)
        {
            scene_op = SceneOp::Replace(Box::new(ClassicGameScene::new(renderer)));
        }

        for _press in conrod_core::widget::Button::new()
            .label("Settings")
            .wh(conrod_core::Dimensions::new(100.0, 100.0))
            .bottom_left_with_margin_on(self.ids.canvas, MARGIN)
            .set(self.ids.settings_button, &mut ui_cell)
        {
            println!("Settings");
        }

        for _press in conrod_core::widget::Button::new()
            .label("Guide")
            .wh(conrod_core::Dimensions::new(100.0, 100.0))
            .mid_bottom_with_margin_on(self.ids.canvas, MARGIN)
            .set(self.ids.guide_button, &mut ui_cell)
        {
            println!("Guide");
        }

        for _press in conrod_core::widget::Button::new()
            .label("Quit")
            .wh(conrod_core::Dimensions::new(100.0, 100.0))
            .bottom_right_with_margin_on(self.ids.canvas, MARGIN)
            .set(self.ids.quit_button, &mut ui_cell)
        {
            *control_flow = ControlFlow::Exit;
        }

        scene_op
    }
}

pub struct GamePhysics {
    gravity: nalgebra::Vector3<f32>,
    rigid_body_set: rapier3d::prelude::RigidBodySet,
    collider_set: rapier3d::prelude::ColliderSet,
    integration_parameters: rapier3d::prelude::IntegrationParameters,
    physics_pipeline: rapier3d::prelude::PhysicsPipeline,
    island_manager: rapier3d::prelude::IslandManager,
    broad_phase: rapier3d::prelude::BroadPhase,
    narrow_phase: rapier3d::prelude::NarrowPhase,
    joint_set: rapier3d::prelude::JointSet,
    ccd_solver: rapier3d::prelude::CCDSolver,
}

impl GamePhysics {
    fn new() -> Self {
        Self {
            gravity: nalgebra::Vector3::new(0.0, -9.81, 0.0),
            rigid_body_set: rapier3d::prelude::RigidBodySet::new(),
            collider_set: rapier3d::prelude::ColliderSet::new(),
            integration_parameters: rapier3d::prelude::IntegrationParameters::default(),
            physics_pipeline: rapier3d::prelude::PhysicsPipeline::new(),
            island_manager: rapier3d::prelude::IslandManager::new(),
            broad_phase: rapier3d::prelude::BroadPhase::new(),
            narrow_phase: rapier3d::prelude::NarrowPhase::new(),
            joint_set: rapier3d::prelude::JointSet::new(),
            ccd_solver: rapier3d::prelude::CCDSolver::new(),
        }
    }
}

pub struct ClassicGameScene {
    world: World,
    physics: GamePhysics,
}

impl ClassicGameScene {
    pub fn new(renderer: &mut Renderer) -> Self {
        renderer.is_render_gui = false;
        renderer.is_render_game = true;

        let world = World::new();
        Self {
            world,
            physics: GamePhysics::new(),
        }
    }
}

impl Scene for ClassicGameScene {
    fn update(
        &mut self,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        control_flow: &mut ControlFlow,
    ) -> SceneOp {
        if input_manager.is_keyboard_press(&VirtualKeyCode::A) {
            renderer.camera.position -= 3.0 * delta_time * renderer.camera.get_direction_right();
        } else if input_manager.is_keyboard_press(&VirtualKeyCode::D) {
            renderer.camera.position += 3.0 * delta_time * renderer.camera.get_direction_right();
        }

        if input_manager.is_keyboard_press(&VirtualKeyCode::W) {
            renderer.camera.position +=
                3.0 * delta_time * renderer.camera.get_direction_without_pitch();
        } else if input_manager.is_keyboard_press(&VirtualKeyCode::S) {
            renderer.camera.position -=
                3.0 * delta_time * renderer.camera.get_direction_without_pitch();
        }

        SceneOp::None
    }
}
