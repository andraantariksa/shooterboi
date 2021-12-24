use gluesql::data::Value;
use std::collections::HashMap;

use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::timer::Timer;
use crate::window::Window;

pub mod classic_game_scene;
pub mod classic_score_scene;
pub mod elimination_game_scene;
pub mod elimination_score_scene;
pub mod exit_confirm_scene;
pub mod game_selection_scene;
pub mod guide_scene;
pub mod hit_and_dodge_scene;
pub mod hit_and_dodge_score_scene;
pub mod main_menu_scene;
pub mod pause_scene;
pub mod scores_scene;
pub mod settings_scene;

pub const MAX_RAYCAST_DISTANCE: f32 = 1000.0;

const BUTTON_WIDTH: f64 = 160.0;
const BUTTON_HEIGHT: f64 = 40.0;

pub const MARGIN: conrod_core::Scalar = 15.0;
pub const GAP_BETWEEN_ITEM: f64 = 25.0;

pub const PREPARE_DURATION: f32 = 3.0;
pub const FINISHING_DURATION: f32 = 3.0;

pub type Message = HashMap<&'static str, Value>;
pub type MaybeMessage = Option<Message>;

pub enum GameState {
    Preround,
    Prepare(Timer),
    Round,
    Finishing(Timer),
}

pub enum SceneOp {
    None,
    Push(Box<dyn Scene>, MaybeMessage),
    Pop(u8, MaybeMessage),
    Replace(Box<dyn Scene>, MaybeMessage),
}

pub trait Scene {
    fn init(
        &mut self,
        message: MaybeMessage,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        database: &mut Database,
    );

    fn update(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        control_flow: &mut ControlFlow,
        database: &mut Database,
    ) -> SceneOp;

    fn prerender(
        &mut self,
        _renderer: &mut Renderer,
        _input_manager: &InputManager,
        _delta_time: f32,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
    ) {
    }

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        database: &mut Database,
    );
}
