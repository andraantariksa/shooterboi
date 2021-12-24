use conrod_core::widget::envelope_editor::EnvelopePoint;

use conrod_core::widget::drop_down_list::Idx;
use conrod_core::widget::{Button, Canvas, DropDownList, List, Text};
use conrod_core::{Dimensions, Labelable, Positionable, Sizeable, Widget};
use std::collections::HashMap;

use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;

use crate::scene::classic_score_scene::ClassicGameScoreDisplay;

use crate::scene::{
    MaybeMessage, Scene, SceneOp, Value, BUTTON_HEIGHT, BUTTON_WIDTH, GAP_BETWEEN_ITEM, MARGIN,
};
use crate::window::Window;
use conrod_core::widget_ids;

use winit::event::VirtualKeyCode;

widget_ids! {
    pub struct ScoresSceneIds {
        canvas,

        header_canvas,

        body_canvas,
        score_list,

        footer_canvas,

        mode_selection,

        difficulty_selection,

        back_button,

        title_text,
    }
}

struct GameMode {
    image: conrod_core::image::Id,
    title: &'static str,
    description: &'static str,
}

pub struct ScoresScene {
    ids: ScoresSceneIds,
    scores: Vec<ClassicGameScoreDisplay>,
    mode_selection: Idx,
    difficulty_selection: Idx,
}

impl ScoresScene {
    pub fn new(_renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        Self {
            ids: ScoresSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
            scores: vec![],
            mode_selection: 0,
            difficulty_selection: 0,
        }
    }
}

impl Scene for ScoresScene {
    fn init(
        &mut self,
        _message: MaybeMessage,
        _window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        database: &mut Database,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;

        self.scores = database.read_classic_game_score();
    }

    fn update(
        &mut self,
        _window: &mut Window,
        _renderer: &mut Renderer,
        input_manager: &InputManager,
        _delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        _control_flow: &mut ControlFlow,
        _database: &mut Database,
    ) -> SceneOp {
        let mut scene_op = SceneOp::None;

        let back_button;
        {
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

            Canvas::new()
                .flow_down(&[
                    (
                        self.ids.header_canvas,
                        Canvas::new().length(BUTTON_HEIGHT + MARGIN * 2.0),
                    ),
                    (self.ids.body_canvas, Canvas::new().length_weight(1.0)),
                    (
                        self.ids.footer_canvas,
                        Canvas::new().length(BUTTON_HEIGHT + MARGIN * 2.0),
                    ),
                ])
                .set(self.ids.canvas, &mut ui_cell);

            let (mut score_list_event, score_list_scrollbar) = List::flow_down(self.scores.len())
                .wh_of(self.ids.body_canvas)
                .item_size(150.0)
                .scrollbar_color(conrod_core::color::RED)
                .scrollbar_next_to()
                .middle_of(self.ids.body_canvas)
                .set(self.ids.score_list, &mut ui_cell);
            if let Some(s) = score_list_scrollbar {
                s.set(&mut ui_cell)
            }
            while let Some(item) = score_list_event.next(&ui_cell) {
                let s = format!("{}", self.scores[item.i]);
                let text = Text::new(&s);
                item.set(text, &mut ui_cell);
            }

            const MODES: &[&str; 3] = &["Classic", "Elimination", "Hit and Dodge"];
            const DIFFICULTY: &[&str; 3] = &["Easy", "Medium", "Hard"];

            if let Some(new_idx) = DropDownList::new(MODES, Some(self.difficulty_selection))
                .wh(Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .top_left_with_margin_on(self.ids.header_canvas, MARGIN)
                .scrollbar_next_to()
                .set(self.ids.mode_selection, &mut ui_cell)
            {
                self.difficulty_selection = new_idx;
            }

            if let Some(new_idx) = DropDownList::new(DIFFICULTY, Some(self.mode_selection))
                .wh(Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .right_from(self.ids.mode_selection, GAP_BETWEEN_ITEM)
                .scrollbar_next_to()
                .set(self.ids.difficulty_selection, &mut ui_cell)
            {
                self.mode_selection = new_idx;
            }

            back_button = Button::new()
                .label("Back")
                .wh(Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .bottom_left_with_margin_on(self.ids.footer_canvas, MARGIN)
                .set(self.ids.back_button, &mut ui_cell);

            Text::new("Score")
                .right_justify()
                .bottom_right_with_margin_on(self.ids.footer_canvas, MARGIN * 2.0)
                .set(self.ids.title_text, &mut ui_cell);
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
        _database: &mut Database,
    ) {
    }
}
