use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Labelable, Positionable, Sizeable, Widget};
use std::io::{BufReader, Cursor};
use winit::event_loop::ControlFlow;

use crate::audio::{AudioContext, AUDIO_FILE_AWESOMENESS};
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::scene::classic_game_scene::ClassicGameScene;
use crate::scene::exit_confirm_scene::QuitConfirmScene;
use crate::scene::guide_scene::GuideScene;
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
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;

        println!("Init main menu");
        if let Some(message) = message {
            if let Some(is_start) = message.get("start_bgm") {
                if let Value::Bool(false) = is_start {
                } else {
                    let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
                    sink.append(
                        rodio::Decoder::new(BufReader::new(Cursor::new(
                            AUDIO_FILE_AWESOMENESS.to_vec(),
                        )))
                        .unwrap()
                        .repeat_infinite(),
                    );
                    audio_context.global_sinks_map.insert("bgm", sink);
                }
            } else {
                let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
                sink.append(
                    rodio::Decoder::new(BufReader::new(Cursor::new(
                        AUDIO_FILE_AWESOMENESS.to_vec(),
                    )))
                    .unwrap()
                    .repeat_infinite(),
                );
                audio_context.global_sinks_map.insert("bgm", sink);
            }
        } else {
            let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
            sink.append(
                rodio::Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE_AWESOMENESS.to_vec())))
                    .unwrap()
                    .repeat_infinite(),
            );
            audio_context.global_sinks_map.insert("bgm", sink);
        }
    }

    fn update(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        _delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        _control_flow: &mut ControlFlow,
    ) -> SceneOp {
        let mut scene_op = SceneOp::None;

        let settings_button;
        let classic_game_button;
        let mut quit_button;
        let guide_button;

        {
            let image_id = *conrod_handle.get_image_id_map().get("title").unwrap();
            let ropa_font_id = *conrod_handle.get_font_id_map().get("ropa").unwrap();
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

            conrod_core::widget::Canvas::new().set(self.ids.canvas, &mut ui_cell);

            quit_button = conrod_core::widget::Button::new()
                .label("Quit")
                .label_font_id(ropa_font_id)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .bottom_left_with_margin_on(self.ids.canvas, MARGIN)
                .set(self.ids.quit_button, &mut ui_cell);

            let canvas_w = ui_cell.w_of(self.ids.canvas).unwrap();
            let button_w = ui_cell.w_of(self.ids.quit_button).unwrap();

            const GAP_BETWEEN_BUTTON: f64 = 20.0;

            conrod_core::widget::Image::new(image_id)
                .bottom_right_with_margin_on(self.ids.canvas, MARGIN)
                .w(canvas_w - button_w - MARGIN * 2.0)
                .set(self.ids.title_image, &mut ui_cell);

            classic_game_button = conrod_core::widget::Button::new()
                .label("Classic Mode")
                .label_font_id(ropa_font_id)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .up_from(self.ids.guide_button, GAP_BETWEEN_BUTTON)
                .set(self.ids.start_classic_mode_button, &mut ui_cell);

            guide_button = conrod_core::widget::Button::new()
                .label("Guide")
                .label_font_id(ropa_font_id)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .up_from(self.ids.settings_button, GAP_BETWEEN_BUTTON)
                .set(self.ids.guide_button, &mut ui_cell);

            settings_button = conrod_core::widget::Button::new()
                .label("Settings")
                .label_font_id(ropa_font_id)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .up_from(self.ids.quit_button, GAP_BETWEEN_BUTTON)
                .set(self.ids.settings_button, &mut ui_cell);
        }

        for _press in guide_button {
            scene_op = SceneOp::Push(Box::new(GuideScene::new(renderer, conrod_handle)), None);
        }

        if input_manager.is_keyboard_pressed(&VirtualKeyCode::Escape)
            || quit_button.next().is_some()
        {
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
                Box::new(ClassicGameScene::new(renderer, conrod_handle)),
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
    ) {
        println!("Deinit main menu");
    }
}
