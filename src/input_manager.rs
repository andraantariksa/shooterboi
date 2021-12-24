use crate::window::Window;
use std::collections::HashSet;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

pub struct InputManager {
    pub keyboard_buttons: HashSet<VirtualKeyCode>,
    pub keyboard_buttons_pressed: HashSet<VirtualKeyCode>,
    pub mouse_buttons: HashSet<MouseButton>,
    pub mouse_buttons_pressed: HashSet<MouseButton>,
    pub mouse_movement: nalgebra::Vector2<f32>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keyboard_buttons_pressed: HashSet::new(),
            keyboard_buttons: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons: HashSet::new(),
            mouse_movement: nalgebra::Vector2::new(0.0, 0.0),
        }
    }

    pub fn process(&mut self, event: &WindowEvent, window: &Window) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                #[cfg(not(target_arch = "wasm32"))]
                if window.is_cursor_grabbed() {
                    let window_size = window.inner_size();
                    let center = nalgebra::Vector2::<f32>::new(
                        window_size.width as f32 / 2.0,
                        window_size.height as f32 / 2.0,
                    );

                    let new_pos =
                        nalgebra::Vector2::<f32>::new(position.x as f32, position.y as f32);

                    self.mouse_movement += center - new_pos;

                    window
                        .set_cursor_position(PhysicalPosition {
                            x: center.x,
                            y: center.y,
                        })
                        .unwrap();
                }
                true
            }
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
                self.keyboard_buttons_pressed.insert(*key);
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
            WindowEvent::MouseInput {
                button,
                state: ElementState::Pressed,
                ..
            } => {
                self.mouse_buttons.insert(*button);
                self.mouse_buttons_pressed.insert(*button);
                true
            }
            WindowEvent::MouseInput {
                button,
                state: ElementState::Released,
                ..
            } => {
                self.mouse_buttons.remove(button);
                true
            }
            _ => false,
        }
    }

    pub fn clear(&mut self) {
        self.mouse_movement.data.0 = [[0.0, 0.0]];
        self.keyboard_buttons_pressed.clear();
        self.mouse_buttons_pressed.clear();
    }

    pub fn is_mouse_press(&self, key: &MouseButton) -> bool {
        self.mouse_buttons.contains(key)
    }

    pub fn is_mouse_pressed(&self, key: &MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(key)
    }

    pub fn is_keyboard_press(&self, key: &VirtualKeyCode) -> bool {
        self.keyboard_buttons.contains(key)
    }

    pub fn is_keyboard_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.keyboard_buttons_pressed.contains(key)
    }

    pub fn is_any_press(&self) -> bool {
        !self.keyboard_buttons.is_empty() || !self.mouse_buttons.is_empty()
    }

    pub fn is_any_keyboard_press(&self) -> bool {
        !self.keyboard_buttons.is_empty()
    }

    pub fn is_any_mouse_press(&self) -> bool {
        !self.mouse_buttons.is_empty()
    }
}
