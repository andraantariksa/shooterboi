use crate::audio::AudioContext;
use crate::input_manager::InputManager;
use crate::renderer::rendering_info::RenderingInfo;
use crate::resources::time::DeltaTime;
use winit::event::{MouseButton, VirtualKeyCode};

pub fn player(rendering_info: &RenderingInfo) {}

pub fn shoot(input_manager: &InputManager, audio_context: &mut AudioContext) {
    if input_manager.is_mouse_press(&MouseButton::Left) {}
}
