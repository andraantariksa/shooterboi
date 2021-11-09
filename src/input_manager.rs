use crate::nalgebra;
use std::collections::HashSet;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

pub struct InputManager {
    pub keyboard_buttons: HashSet<VirtualKeyCode>,
    pub mouse_buttons: HashSet<MouseButton>,
    pub mouse_movement: nalgebra::Vector2<f32>,
}

impl InputManager {
    pub(crate) fn new() -> Self {
        Self {
            keyboard_buttons: HashSet::new(),
            mouse_buttons: HashSet::new(),
            mouse_movement: nalgebra::Vector2::new(0.0, 0.0),
        }
    }

    pub(crate) fn process(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.keyboard_buttons.insert(*key);
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Released,
                        ..
                    },
                ..
            } => {
                self.keyboard_buttons.remove(key);
                true
            }
            WindowEvent::MouseInput { button, .. } => {
                self.mouse_buttons.insert(*button);
                true
            }
            _ => false,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.mouse_buttons.clear();
        self.mouse_movement.data.0 = [[0.0, 0.0]];
    }

    pub(crate) fn is_mouse_press(&self, key: &MouseButton) -> bool {
        self.mouse_buttons.contains(key)
    }

    pub(crate) fn is_keyboard_press(&self, key: &VirtualKeyCode) -> bool {
        self.keyboard_buttons.contains(key)
    }
}
