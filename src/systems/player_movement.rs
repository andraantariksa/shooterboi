use crate::camera::Camera;
use crate::input_manager::InputManager;
use nalgebra::Vector3;
use winit::event::VirtualKeyCode;

const SPEED: f32 = 5.0;

pub fn update_player_position(delta_time: f32, input_manager: &InputManager, camera: &mut Camera) {
    if input_manager.is_keyboard_press(&VirtualKeyCode::A) {
        camera.position -= SPEED * delta_time * *camera.get_direction_right();
    } else if input_manager.is_keyboard_press(&VirtualKeyCode::D) {
        camera.position += SPEED * delta_time * *camera.get_direction_right();
    }

    if input_manager.is_keyboard_press(&VirtualKeyCode::W) {
        camera.position += SPEED * delta_time * *camera.get_direction_without_pitch();
    } else if input_manager.is_keyboard_press(&VirtualKeyCode::S) {
        camera.position -= SPEED * delta_time * *camera.get_direction_without_pitch();
    }
}
