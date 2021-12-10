use crate::camera::Camera;
use crate::input_manager::InputManager;
use crate::physics::GamePhysics;
use nalgebra::Vector3;
use rapier3d::prelude::RigidBodyHandle;
use winit::event::VirtualKeyCode;

const SPEED: f32 = 5.0;

pub fn update_player_position(
    delta_time: f32,
    input_manager: &InputManager,
    camera: &mut Camera,
    physics: &mut GamePhysics,
    player_rigid_body_handle: RigidBodyHandle,
) -> Vector3<f32> {
    let player_rigid_body = physics
        .rigid_body_set
        .get_mut(player_rigid_body_handle)
        .unwrap();

    camera.position = *player_rigid_body.translation();

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

    player_rigid_body.set_translation(camera.position, true);
    camera.position
}
