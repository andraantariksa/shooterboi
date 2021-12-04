use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::widget::list_select::Event;
use conrod_core::widget::{Button, Canvas, List, Text};
use conrod_core::{Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use winit::event_loop::ControlFlow;

use crate::audio::{AudioContext, Sink, AUDIO_FILE_AWESOMENESS};
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::scene::classic_game_scene::ClassicGameScene;
use crate::scene::classic_game_scores_scene::ClassicGameScoresScene;
use crate::scene::exit_confirm_scene::QuitConfirmScene;
use crate::scene::guide_scene::GuideScene;
use crate::scene::settings_scene::SettingsScene;
use crate::scene::{MaybeMessage, Scene, SceneOp, Value, BUTTON_HEIGHT, BUTTON_WIDTH, MARGIN};
use crate::window::Window;
use conrod_core::widget_ids;
use rodio::Source;
use winit::event::VirtualKeyCode;

widget_ids! {
    pub struct GameSelectionSceneIds {
        // The main canvas
        canvas,

        body_canvas,

        game_mode_selection_canvas,
        game_mode_selection_listselect,

        game_detail_canvas,

        game_score_canvas,

        footer_canvas,
        back_button,

        title_text_canvas,
        title_text,
        description_text_canvas,
        description_text,
        buttons_canvas,
        play_button_canvas,
        play_button,
        score_button_canvas,
        score_button
    }
}

struct GameMode {
    image: conrod_core::image::Id,
    title: &'static str,
    description: &'static str,
}

pub struct GameSelectionScene {
    ids: GameSelectionSceneIds,
    selected_game_mode_idx: usize,
}

impl GameSelectionScene {
    pub fn new(_renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: GameSelectionSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
            selected_game_mode_idx: 0,
        }
    }
}

impl Scene for GameSelectionScene {
    fn init(
        &mut self,
        message: MaybeMessage,
        _window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        database: &mut Database,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;
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
        database: &mut Database,
    ) -> SceneOp {
        let mut scene_op = SceneOp::None;

        let mut back_button;
        let mut play_button;
        let mut score_button;

        let image_id = *conrod_handle.get_image_id_map().get("title").unwrap();

        let mut game_modes = [
            GameMode {
                image: image_id,
                title: "Classic",
                description: "You have to shoot every spawned target (colored red). The target will be disappeared after you shoot it and another target will be spawned.",
            },
            GameMode {
                image: image_id,
                title: "Elimination",
                description: "The multiple target will appear. The player needs to shoot all of the target before appear too much.",
            },
            GameMode {
                image: image_id,
                title: "Move and shoot",
                description: "You have to move and shoot the target.",
            }];

        {
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

            Canvas::new()
                .flow_down(&[
                    (
                        self.ids.body_canvas,
                        Canvas::new().flow_right(&[
                            (self.ids.game_mode_selection_canvas, Canvas::new()),
                            (
                                self.ids.game_detail_canvas,
                                Canvas::new().flow_down(&[
                                    (self.ids.title_text_canvas, Canvas::new().length(60.0)),
                                    (
                                        self.ids.description_text_canvas,
                                        Canvas::new().length(200.0),
                                    ),
                                    (
                                        self.ids.buttons_canvas,
                                        Canvas::new()
                                            .flow_right(&[
                                                (self.ids.play_button_canvas, Canvas::new()),
                                                (self.ids.score_button_canvas, Canvas::new()),
                                            ])
                                            .length(BUTTON_HEIGHT + 10.0),
                                    ),
                                ]),
                            ),
                        ]),
                    ),
                    (self.ids.footer_canvas, Canvas::new().length(60.0)),
                ])
                .set(self.ids.canvas, &mut ui_cell);

            let (mut game_list_events, game_list_scroll) =
                conrod_core::widget::ListSelect::single(game_modes.len())
                    .flow_down()
                    .wh_of(self.ids.game_mode_selection_canvas)
                    .item_size(100.0)
                    .scrollbar_color(conrod_core::color::RED)
                    .scrollbar_next_to()
                    .middle_of(self.ids.game_mode_selection_canvas)
                    .set(self.ids.game_mode_selection_listselect, &mut ui_cell);
            game_list_scroll.map(|s| s.set(&mut ui_cell));
            while let Some(event) =
                game_list_events.next(&ui_cell, |i| i == self.selected_game_mode_idx)
            {
                use conrod_core::widget::list_select::Event;
                match event {
                    Event::Item(item) => {
                        let game_mode = &game_modes[item.i];
                        let color = if item.i == self.selected_game_mode_idx {
                            conrod_core::color::rgb(190.0 / 255.0, 201.0 / 255.0, 170.0 / 255.0)
                        } else {
                            conrod_core::color::LIGHT_CHARCOAL
                        };
                        let button = Button::new()
                            .label(game_mode.title)
                            .color(color)
                            .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT));
                        item.set(button, &mut ui_cell);
                    }
                    Event::Selection(selection) => {
                        self.selected_game_mode_idx = selection;
                    }
                    _ => {}
                }
            }

            back_button = Button::new()
                .label("Back")
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .bottom_left_with_margin_on(self.ids.footer_canvas, MARGIN)
                .set(self.ids.back_button, &mut ui_cell);

            Text::new(game_modes[self.selected_game_mode_idx].title)
                .middle_of(self.ids.title_text_canvas)
                .padded_w_of(self.ids.title_text_canvas, MARGIN)
                .set(self.ids.title_text, &mut ui_cell);
            Text::new(game_modes[self.selected_game_mode_idx].description)
                .middle_of(self.ids.description_text_canvas)
                .padded_w_of(self.ids.description_text_canvas, MARGIN)
                .h_of(self.ids.description_text_canvas)
                .set(self.ids.description_text, &mut ui_cell);
            play_button = Button::new()
                .label("Play")
                .middle_of(self.ids.play_button_canvas)
                .padded_w_of(self.ids.play_button_canvas, MARGIN)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .set(self.ids.play_button, &mut ui_cell);
            score_button = Button::new()
                .label("Score")
                .middle_of(self.ids.score_button_canvas)
                .padded_w_of(self.ids.score_button_canvas, MARGIN)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .set(self.ids.score_button, &mut ui_cell);
        }

        if play_button.was_clicked() {
            scene_op = SceneOp::Push(
                match self.selected_game_mode_idx {
                    0 => Box::new(ClassicGameScene::new(renderer, conrod_handle)),
                    _ => unreachable!(),
                },
                None,
            );
        }

        if score_button.was_clicked() {
            scene_op = SceneOp::Push(
                match self.selected_game_mode_idx {
                    0 => Box::new(ClassicGameScoresScene::new(renderer, conrod_handle)),
                    _ => unreachable!(),
                },
                None,
            );
        }

        if input_manager.is_keyboard_pressed(&VirtualKeyCode::Escape) || back_button.was_clicked() {
            scene_op = SceneOp::Pop(1, {
                let mut m = HashMap::new();
                m.insert("start_bgm", Value::Bool(false));
                Some(m)
            });
        }

        scene_op
    }

    fn deinit(
        &mut self,
        _window: &mut Window,
        _renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        database: &mut Database,
    ) {
    }
}
