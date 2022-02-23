use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::widget::{Button, Canvas, Image};
use conrod_core::{Labelable, Positionable, Sizeable, Widget};
use std::io::{BufReader, Cursor};
use winit::event_loop::ControlFlow;

use crate::audio::{AudioContext, Sink, AUDIO_FILE_AWESOMENESS};
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;

use crate::scene::exit_confirm_scene::QuitConfirmScene;
use crate::scene::game_selection_scene::GameSelectionScene;

use crate::scene::settings_scene::SettingsScene;
use crate::scene::{MaybeMessage, Scene, SceneOp, Value, BUTTON_HEIGHT, BUTTON_WIDTH, MARGIN};
use crate::window::Window;
use conrod_core::widget_ids;
use rodio::Source;
use winit::event::VirtualKeyCode;

widget_ids! {
    pub struct MainMenuSceneIds {
        // The main canvas
        canvas,
        quit_button,
        guide_button,
        settings_button,
        start_classic_mode_button,
        title_image
    }
}

pub struct MainMenuScene {
    ids: MainMenuSceneIds,
}

pub fn play_bgm(message: &MaybeMessage, audio_context: &mut AudioContext) {
    let play_bgm = || {
        if let Some(ref msg) = message {
            let value = msg.get("start_bgm")?;
            Some(match value {
                Value::Bool(x) => *x,
                _ => unreachable!(),
            })
        } else {
            None
        }
    };
    if play_bgm().is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
            sink.append(
                rodio::Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE_AWESOMENESS.to_vec())))
                    .unwrap()
                    .repeat_infinite(),
            );
            sink.set_volume(audio_context.get_volume());
            audio_context
                .global_sinks_map
                .insert("bgm", Sink::Regular(sink));
        }
    }
}

impl MainMenuScene {
    pub fn new(_renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: MainMenuSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
        }
    }
}

impl Scene for MainMenuScene {
    fn init(
        &mut self,
        message: MaybeMessage,
        _window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        _database: &mut Database,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;

        play_bgm(&message, audio_context);
    }

    fn update(
        &mut self,
        _window: &mut Window,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        _delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        _control_flow: &mut ControlFlow,
        _database: &mut Database,
    ) -> SceneOp {
        let mut scene_op = SceneOp::None;

        let settings_button;
        let classic_game_button;
        #[cfg(not(target_arch = "wasm32"))]
        let mut quit_button;
        // let guide_button;

        {
            let image_id = *conrod_handle.get_image_id_map().get("title").unwrap();
            let image = conrod_handle.get_image_map().get(&image_id).unwrap();
            let ratio = image.height as f64 / image.width as f64;
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

            Canvas::new().set(self.ids.canvas, &mut ui_cell);

            #[cfg(not(target_arch = "wasm32"))]
            {
                quit_button = Button::new()
                    .label("Quit")
                    .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .bottom_left_with_margin_on(self.ids.canvas, MARGIN)
                    .set(self.ids.quit_button, &mut ui_cell);
            }

            let canvas_w = ui_cell.w_of(self.ids.canvas).unwrap();

            const GAP_BETWEEN_BUTTON: f64 = 20.0;

            let mut _settings_button = Button::new()
                .label("Settings")
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT));
            if cfg!(target_arch = "wasm32") {
                _settings_button =
                    _settings_button.bottom_left_with_margin_on(self.ids.canvas, MARGIN);
            } else {
                _settings_button =
                    _settings_button.up_from(self.ids.quit_button, GAP_BETWEEN_BUTTON);
            }
            settings_button = _settings_button.set(self.ids.settings_button, &mut ui_cell);

            // guide_button = Button::new()
            //     .label("Guide")
            //     .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
            //     .up_from(self.ids.settings_button, GAP_BETWEEN_BUTTON)
            //     .set(self.ids.guide_button, &mut ui_cell);

            classic_game_button = Button::new()
                .label("Play")
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .up_from(self.ids.settings_button, GAP_BETWEEN_BUTTON)
                // .up_from(self.ids.guide_button, GAP_BETWEEN_BUTTON)
                .set(self.ids.start_classic_mode_button, &mut ui_cell);

            let logo_w = canvas_w - BUTTON_WIDTH - MARGIN * 2.0;
            Image::new(image_id)
                .bottom_right_with_margin_on(self.ids.canvas, MARGIN)
                .w(logo_w)
                .h(ratio * logo_w)
                .set(self.ids.title_image, &mut ui_cell);
        }

        // for _press in guide_button {
        //     scene_op = SceneOp::Push(Box::new(GuideScene::new(renderer, conrod_handle)), None);
        // }

        if input_manager.is_keyboard_pressed(&VirtualKeyCode::Escape) || {
            #[cfg(not(target_arch = "wasm32"))]
            let ret = quit_button.next().is_some();
            #[cfg(target_arch = "wasm32")]
            let ret = false;
            ret
        } {
            scene_op = SceneOp::Push(
                Box::new(QuitConfirmScene::new(renderer, conrod_handle)),
                None,
            );
        }

        for _press in settings_button {
            scene_op = SceneOp::Push(Box::new(SettingsScene::new(renderer, conrod_handle)), None);
        }

        for _press in classic_game_button {
            scene_op = SceneOp::Push(
                Box::new(GameSelectionScene::new(renderer, conrod_handle)),
                None,
            );
        }

        scene_op
    }

    fn deinit(
        &mut self,
        _window: &mut Window,
        _renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        _database: &mut Database,
    ) {
    }
}
