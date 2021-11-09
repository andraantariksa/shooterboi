use crate::nalgebra::Point2;
use crate::nalgebra::Vector3;
use ambisonic::{rodio, AmbisonicBuilder};
use hecs::World;
use instant::Instant;
use rapier3d::na::Vector2;
use rapier3d::prelude::*;
use std::iter;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalPosition;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::audio::AudioContext;
use crate::camera::Camera;
use crate::components::{Target, Transform};
use crate::input_manager::InputManager;
use crate::renderer::rendering_info::RenderingInfo;
use crate::resources::time::DeltaTime;
use state::State;

mod audio;
mod camera;
mod components;
mod input_manager;
mod renderer;
mod resources;
mod state;
mod systems;
mod util;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Warn);
    }
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        body.append_child(&canvas)
            .expect("Append canvas to HTML body");
    }

    // Physic
    let gravity = vector![0.0, -9.81, 0.0];
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();

    // Ground
    let ground_rigid_body_handle =
        rigid_body_set.insert(RigidBodyBuilder::new(RigidBodyType::Static).build());
    collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::cuboid(999.999, 0.01, 999.999)).build(),
        ground_rigid_body_handle,
        &mut rigid_body_set,
    );

    // ECS
    let mut world = World::new();

    // Enemy
    world.spawn((Transform::builder(), Target));

    // Player
    let player_rigid_body_handle = rigid_body_set.insert(
        RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .translation(Vector3::new(0.0, 3.0, 0.0))
            .build(),
    );
    collider_set.insert_with_parent(
        ColliderBuilder::new(SharedShape::cuboid(0.5, 0.5, 0.5)).build(),
        player_rigid_body_handle,
        &mut rigid_body_set,
    );
    world.spawn((player_rigid_body_handle,));

    let mut audio_context = AudioContext::new();
    let mut input_manager = InputManager::new();
    let mut rendering_info = RenderingInfo::new(window.inner_size());

    let mut state = pollster::block_on(State::new(&window, &rendering_info));

    let mut game_starting_time = Instant::now();
    let mut game_last_iter = Instant::now();

    let mut camera = Camera::new();

    // window.set_cursor_position(PhysicalPosition {
    //     x: state.window_size.width as f32 / 2.0,
    //     y: state.window_size.height as f32 / 2.0
    // });

    window.set_cursor_grab(true);
    window.set_cursor_visible(false);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                input_manager.process(&event);
                if !state.input(&event) {
                    match event {
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::P),
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => {
                            // let source = rodio::source::SineWave::new(440);
                            // let mut sound = scene.play_at(source, [50.0, 1.0, 0.0]);
                            // log::error!("play sound");
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                let center = Point2::<f32>::new(
                                    state.window_size.width as f32 / 2.0,
                                    state.window_size.height as f32 / 2.0,
                                );
                                let new_pos =
                                    Point2::<f32>::new(position.x as f32, position.y as f32);

                                camera.move_direction(center - new_pos);
                            }

                            window.set_cursor_position(PhysicalPosition {
                                x: state.window_size.width as f32 / 2.0,
                                y: state.window_size.height as f32 / 2.0,
                            });
                        }
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            rendering_info.reso_time.x = physical_size.width as f32;
                            rendering_info.reso_time.y = physical_size.height as f32;
                            state.resize(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            rendering_info.reso_time.x = new_inner_size.width as f32;
                            rendering_info.reso_time.y = new_inner_size.height as f32;
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(*new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::MainEventsCleared => {
                let duration = game_last_iter.elapsed().as_secs_f32();
                game_last_iter = Instant::now();
                let delta_time = duration;

                {
                    let mut player_rigid_body =
                        rigid_body_set.get_mut(player_rigid_body_handle).unwrap();
                    camera.position = *player_rigid_body.translation();

                    {
                        #[cfg(target_arch = "wasm32")]
                        {
                            let mut dir_diff = Vector2::new(0.0, 0.0);
                            if input_manager.is_keyboard_press(&VirtualKeyCode::Left) {
                                dir_diff.x += 10.0 * delta_time;
                            } else if input_manager.is_keyboard_press(&VirtualKeyCode::Right) {
                                dir_diff.x -= 10.0 * delta_time;
                            }

                            if input_manager.is_keyboard_press(&VirtualKeyCode::Up) {
                                dir_diff.y += 10.0 * delta_time;
                            } else if input_manager.is_keyboard_press(&VirtualKeyCode::Down) {
                                dir_diff.y -= 10.0 * delta_time;
                            }

                            camera.move_direction(dir_diff);
                        }

                        if input_manager.is_keyboard_press(&VirtualKeyCode::A) {
                            camera.position -= 3.0 * delta_time * camera.get_direction_right();
                        } else if input_manager.is_keyboard_press(&VirtualKeyCode::D) {
                            camera.position += 3.0 * delta_time * camera.get_direction_right();
                        }

                        if input_manager.is_keyboard_press(&VirtualKeyCode::W) {
                            camera.position +=
                                3.0 * delta_time * camera.get_direction_without_pitch();
                        } else if input_manager.is_keyboard_press(&VirtualKeyCode::S) {
                            camera.position -=
                                3.0 * delta_time * camera.get_direction_without_pitch();
                        }

                        if input_manager.is_keyboard_press(&VirtualKeyCode::Space)
                            // TODO change this
                            && player_rigid_body.linvel().y <= 0.1
                        {
                            player_rigid_body.set_linvel(Vector3::new(0.0, 4.0, 0.0), false);
                        }

                        player_rigid_body.set_translation(camera.position, false);
                        rendering_info.cam_pos = camera.position;
                        rendering_info.cam_dir = camera.get_direction();
                    }

                    // for (id, (transform)) in world.query::<(&Transform)>().iter() {}
                }

                physics_pipeline.step(
                    &gravity,
                    &integration_parameters,
                    &mut island_manager,
                    &mut broad_phase,
                    &mut narrow_phase,
                    &mut rigid_body_set,
                    &mut collider_set,
                    &mut joint_set,
                    &mut ccd_solver,
                    &(),
                    &(),
                );

                let duration = game_starting_time.elapsed();
                rendering_info.reso_time.z = duration.as_secs_f32();

                match state.render(&rendering_info) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.window_size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
                input_manager.clear();
            }
            // Event::RedrawRequested(_) => {
            // }
            // Event::MainEventsCleared => {
            //     // RedrawRequested will only trigger once, unless we manually
            //     // request it.
            //     window.request_redraw();
            // }
            _ => {}
        }
    });
}
