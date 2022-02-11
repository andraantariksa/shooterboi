use crate::audio::AudioContext;
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;

use crate::scene::{main_menu_scene::MainMenuScene, Scene, SceneOp};
use crate::window::Window;
use instant::Instant;
use std::collections::VecDeque;
use std::env;

use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::Window as WinitWindow;

conrod_winit::v023_conversion_fns!();

pub struct Game {
    scene_stack: VecDeque<Box<dyn Scene>>,
    renderer: Renderer,
    last_time: Instant,
    running_time: f32,
    window: Window,
    input_manager: InputManager,
    conrod_handle: ConrodHandle,
    audio_context: AudioContext,
    database: Database,
    debug: bool,
}

impl Game {
    pub fn new(window: WinitWindow) -> Self {
        let mut renderer = pollster::block_on(Renderer::new(&window));
        let mut conrod_handle = ConrodHandle::new(&mut renderer);
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
        let mut window = Window::from(window);
        let mut audio_context = AudioContext::new();
        let mut database = Database::new();
        database.init();
        database.init_settings(&mut audio_context, &mut renderer);

        let mut scene_stack = VecDeque::<Box<dyn Scene>>::new();
        let mut first_scene = MainMenuScene::new(&mut renderer, &mut conrod_handle); // ClassicScoreScene::new(&mut renderer, &mut conrod_handle);
                                                                                     // let mut first_scene = DodgeAndDestroyGameScene::new(&mut renderer, &mut conrod_handle);
        first_scene.init(
            None,
            &mut window,
            &mut renderer,
            &mut conrod_handle,
            &mut audio_context,
            &mut database,
        );
        scene_stack.push_back(Box::new(first_scene));

        let debug = env::var("debug").is_ok();

        Self {
            window,
            scene_stack,
            conrod_handle,
            renderer,
            input_manager: InputManager::new(),
            last_time: Instant::now(),
            running_time: 0.0,
            audio_context,
            database,
            debug,
        }
    }

    pub fn update(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        if let Some(event) = convert_event(event, &self.window) {
            self.conrod_handle.get_ui_mut().handle_event(event);
        }
        match event {
            Event::WindowEvent { event, window_id } if *window_id == self.window.id() => {
                self.input_manager.process(event, &self.window);
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
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
                let current_time = Instant::now();
                let delta_time = current_time.duration_since(self.last_time).as_secs_f32();
                self.last_time = current_time;
                self.running_time += delta_time;

                if self.window.is_cursor_grabbed() {
                    let mut dir_diff = nalgebra::Vector2::new(0.0, 0.0);
                    if self.input_manager.is_keyboard_press(&VirtualKeyCode::Left) {
                        dir_diff.x += 400.0;
                    } else if self.input_manager.is_keyboard_press(&VirtualKeyCode::Right) {
                        dir_diff.x -= 400.0;
                    }

                    if self.input_manager.is_keyboard_press(&VirtualKeyCode::Up) {
                        dir_diff.y += 400.0;
                    } else if self.input_manager.is_keyboard_press(&VirtualKeyCode::Down) {
                        dir_diff.y -= 400.0;
                    }

                    self.input_manager.mouse_movement += dir_diff;
                }

                let scene_op = self.scene_stack.back_mut().unwrap().update(
                    &mut self.window,
                    &mut self.renderer,
                    &self.input_manager,
                    delta_time,
                    &mut self.conrod_handle,
                    &mut self.audio_context,
                    control_flow,
                    &mut self.database,
                );

                match scene_op {
                    SceneOp::None => {}
                    SceneOp::Pop(layer_number, message) => {
                        for _ in 0..layer_number {
                            self.scene_stack.back_mut().unwrap().deinit(
                                &mut self.window,
                                &mut self.renderer,
                                &mut self.conrod_handle,
                                &mut self.audio_context,
                                &mut self.database,
                            );
                            self.scene_stack.pop_back();
                        }
                        self.scene_stack.back_mut().unwrap().init(
                            message,
                            &mut self.window,
                            &mut self.renderer,
                            &mut self.conrod_handle,
                            &mut self.audio_context,
                            &mut self.database,
                        );
                    }
                    SceneOp::Push(mut new_scene, message) => {
                        if let Some(prev_scene) = self.scene_stack.back_mut() {
                            prev_scene.deinit(
                                &mut self.window,
                                &mut self.renderer,
                                &mut self.conrod_handle,
                                &mut self.audio_context,
                                &mut self.database,
                            );
                        }
                        new_scene.init(
                            message,
                            &mut self.window,
                            &mut self.renderer,
                            &mut self.conrod_handle,
                            &mut self.audio_context,
                            &mut self.database,
                        );
                        self.scene_stack.push_back(new_scene);
                    }
                    SceneOp::Replace(mut new_scene, message) => {
                        self.scene_stack.back_mut().unwrap().deinit(
                            &mut self.window,
                            &mut self.renderer,
                            &mut self.conrod_handle,
                            &mut self.audio_context,
                            &mut self.database,
                        );
                        self.scene_stack.pop_back();
                        new_scene.init(
                            message,
                            &mut self.window,
                            &mut self.renderer,
                            &mut self.conrod_handle,
                            &mut self.audio_context,
                            &mut self.database,
                        );
                        self.scene_stack.push_back(new_scene);
                    }
                };

                self.scene_stack.back_mut().unwrap().prerender(
                    &mut self.renderer,
                    &self.input_manager,
                    delta_time,
                    &mut self.conrod_handle,
                    &mut self.audio_context,
                );

                match self
                    .renderer
                    .render(self.running_time, &mut self.conrod_handle)
                {
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
                    Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory"),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(_e) => {}
                };

                self.input_manager.clear();
                self.audio_context.clear();

                if self.debug {
                    self.window.set_title(&format!("FPS: {}", 1.0 / delta_time));
                }
            }
            _ => {}
        };
    }
}
