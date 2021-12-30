use gluesql::data::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::timer::Timer;
use crate::window::Window;

pub mod classic_game_scene;
pub mod elimination_game_scene;
pub mod exit_confirm_scene;
pub mod game_score_scene;
pub mod game_selection_scene;
pub mod guide_scene;
pub mod hit_and_dodge_scene;
pub mod main_menu_scene;
pub mod pause_scene;
pub mod score_history_scene;
pub mod settings_scene;

const BUTTON_WIDTH: f64 = 160.0;
const BUTTON_HEIGHT: f64 = 40.0;

pub const MARGIN: conrod_core::Scalar = 15.0;
pub const GAP_BETWEEN_ITEM: f64 = 25.0;

pub const PREPARE_DURATION: f32 = 3.0;
pub const FINISHING_DURATION: f32 = 3.0;

pub const IN_SHOOT_ANIM_DURATION: f32 = 0.1;
pub const OUT_SHOOT_ANIM_DURATION: f32 = 0.1;

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

#[repr(u8)]
pub enum GameMode {
    Classic = 0,
    Elimination = 1,
    HitAndDodge = 2,
}

impl From<usize> for GameMode {
    fn from(x: usize) -> Self {
        match x {
            0 => GameMode::Classic,
            1 => GameMode::Elimination,
            2 => GameMode::HitAndDodge,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum GameDifficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
}

impl Display for GameDifficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameDifficulty::Easy => "Easy",
                GameDifficulty::Medium => "Medium",
                GameDifficulty::Hard => "Hard",
            }
        )
    }
}

impl From<usize> for GameDifficulty {
    fn from(x: usize) -> Self {
        match x {
            0 => GameDifficulty::Easy,
            1 => GameDifficulty::Medium,
            2 => GameDifficulty::Hard,
            _ => unreachable!(),
        }
    }
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
