use std::collections::HashMap;

use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::window::Window;

pub mod after_game_scene;
pub mod classic_game_scene;
pub mod classic_score_scene;
pub mod exit_confirm_scene;
pub mod guide_scene;
pub mod main_menu_scene;
pub mod pause_scene;
pub mod settings_scene;

const BUTTON_WIDTH: f64 = 160.0;
const BUTTON_HEIGHT: f64 = 40.0;

const GAP_BETWEEN_ITEM: f64 = 25.0;

pub enum Value {
    String(String),
    I32(i32),
    F32(f32),
    Bool(bool),
}

impl Value {
    pub fn to_string(&self) -> &String {
        return match self {
            Value::String(s) => s,
            _ => panic!("Not a string"),
        };
    }

    pub fn to_bool(&self) -> &bool {
        return match self {
            Value::Bool(b) => b,
            _ => panic!("Not a bool"),
        };
    }

    pub fn to_i32(&self) -> &i32 {
        return match self {
            Value::I32(i) => i,
            _ => panic!("Not an i32"),
        };
    }

    pub fn to_f32(&self) -> &f32 {
        return match self {
            Value::F32(f) => f,
            _ => panic!("Not an f32"),
        };
    }
}

pub type Message = HashMap<&'static str, Value>;
pub type MaybeMessage = Option<Message>;

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
    ) -> SceneOp;

    fn prerender(
        &mut self,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
    }

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    );
}

pub const MARGIN: conrod_core::Scalar = 15.0;
