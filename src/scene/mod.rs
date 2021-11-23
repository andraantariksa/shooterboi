use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget};
use hecs::World;
use rapier3d::prelude::*;
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;

use pause_scene::PauseScene;
use settings_scene::SettingsScene;

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

pub enum SceneOp {
    None,
    Push(Box<dyn Scene>),
    Pop(u8),
    Replace(Box<dyn Scene>),
}

pub trait Scene {
    fn init(
        &mut self,
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

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    );
}

pub const MARGIN: conrod_core::Scalar = 30.0;
