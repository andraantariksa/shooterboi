use std::collections::HashMap;

use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::window::Window;

pub mod after_game_scene;
pub mod classic_game_scene;
pub mod exit_confirm_scene;
pub mod guide_scene;
pub mod main_menu_scene;
pub mod pause_scene;
pub mod score_scene;
pub mod settings_scene;

pub enum Value {
    String(String),
    I32(i32),
    F32(f32),
    Bool(bool),
}

pub type MaybeMessage = Option<HashMap<&'static str, Value>>;

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

pub const MARGIN: conrod_core::Scalar = 30.0;
