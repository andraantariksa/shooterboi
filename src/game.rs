use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::scene::{ClassicGameScene, ConrodHandle, MainMenuScene, Scene, SceneOp};
use crate::window::Window;
use instant::Instant;
use std::collections::VecDeque;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window as WinitWindow, WindowId};

conrod_winit::v023_conversion_fns!();

pub struct App {
    scene_stack: VecDeque<Box<dyn Scene>>,
    renderer: Renderer,
    delta_time: Instant,
    app_run_time: Instant,
    window: Window,
    input_manager: InputManager,
    conrod_handle: ConrodHandle,
}

impl App {
    pub fn new(window: WinitWindow) -> Self {
        let mut renderer = pollster::block_on(Renderer::new(&window));
        // self.window.set_is_cursor_grabbed(true);
        let mut conrod_handle = ConrodHandle::new(&renderer);
        conrod_handle.get_ui_mut().handle_event(
            convert_event::<()>(
                &Event::WindowEvent {
                    window_id: window.id(),
                    event: WindowEvent::Resized(window.inner_size()),
                },
                &window,
            )
            .unwrap(),
        );
        let mut scene_stack = VecDeque::<Box<dyn Scene>>::new();
        scene_stack.push_back(Box::new(MainMenuScene::new(
            &mut renderer,
            &mut conrod_handle,
        )));
        Self {
            window: Window::from(window),
            scene_stack,
            conrod_handle,
            renderer,
            input_manager: InputManager::new(),
            delta_time: Instant::now(),
            app_run_time: Instant::now(),
        }
    }

    pub fn update(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        if let Some(event) = convert_event(&event, &self.window) {
            self.conrod_handle.get_ui_mut().handle_event(event);
        }
        match event {
            Event::WindowEvent { event, window_id } if *window_id == self.window.id() => {
                self.input_manager.process(&event);
                match event {
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        #[cfg(not(target_arch = "wasm32"))]
                        if self.window.is_cursor_grabbed() {
                            let window_size = self.window.inner_size();
                            let center = nalgebra::Point2::<f32>::new(
                                window_size.width as f32 / 2.0,
                                window_size.height as f32 / 2.0,
                            );
                            let new_pos =
                                nalgebra::Point2::<f32>::new(position.x as f32, position.y as f32);

                            self.renderer.camera.move_direction(center - new_pos);
                            self.window.set_cursor_position(PhysicalPosition {
                                x: window_size.width as f32 / 2.0,
                                y: window_size.height as f32 / 2.0,
                            });
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        self.renderer
                            .resize(physical_size, self.window.scale_factor());
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.renderer
                            .resize(*new_inner_size, self.window.scale_factor());
                    }
                    _ => {}
                };
            }
            Event::MainEventsCleared => {
                // #[cfg(target_arch = "wasm32")]
                let delta_time = self.delta_time.elapsed().as_secs_f32();
                {
                    let mut dir_diff = nalgebra::Vector2::new(0.0, 0.0);
                    if self.input_manager.is_keyboard_press(&VirtualKeyCode::Left) {
                        dir_diff.x += 10.0 * delta_time;
                    } else if self.input_manager.is_keyboard_press(&VirtualKeyCode::Right) {
                        dir_diff.x -= 10.0 * delta_time;
                    }

                    if self.input_manager.is_keyboard_press(&VirtualKeyCode::Up) {
                        dir_diff.y += 10.0 * delta_time;
                    } else if self.input_manager.is_keyboard_press(&VirtualKeyCode::Down) {
                        dir_diff.y -= 10.0 * delta_time;
                    }

                    self.renderer.camera.move_direction(dir_diff);
                }

                let scene_op = self.scene_stack.back_mut().unwrap().update(
                    &mut self.renderer,
                    &self.input_manager,
                    delta_time,
                    &mut self.conrod_handle,
                    control_flow,
                );
                match self.renderer.render(
                    self.app_run_time.elapsed().as_secs_f32(),
                    &mut self.conrod_handle,
                ) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.renderer.resize(
                        &PhysicalSize {
                            width: self.renderer.surface_and_window_config.surface.width,
                            height: self.renderer.surface_and_window_config.surface.height,
                        },
                        self.window.scale_factor(),
                    ),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                };
                self.input_manager.clear();

                self.delta_time = Instant::now();

                match scene_op {
                    SceneOp::None => {}
                    SceneOp::Pop => {
                        self.scene_stack.pop_back();
                    }
                    SceneOp::Push(new_scene) => {
                        self.scene_stack.push_back(new_scene);
                    }
                    SceneOp::Replace(new_scene) => {
                        self.scene_stack.pop_back();
                        self.scene_stack.push_back(new_scene);
                    }
                };
            }
            _ => {}
        };
    }
}
