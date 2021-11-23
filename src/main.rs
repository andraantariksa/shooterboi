extern crate winit;

mod audio;
mod camera;
mod game;
mod gui;
mod input_manager;
mod physics;
mod renderer;
mod scene;
mod timer;
mod util;
mod window;

use crate::game::Game;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Warn);
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Shooterboi")
        .build(&event_loop)
        .unwrap();

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

    let mut game = Game::new(window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        game.update(&event, control_flow);
    });
}
