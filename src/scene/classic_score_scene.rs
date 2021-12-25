use chrono::NaiveDateTime;
use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::widget::Text;
use conrod_core::{Labelable, Positionable, Sizeable, Widget};
use std::fmt::{Display, Formatter};

use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;

use crate::scene::{
    MaybeMessage, Scene, SceneOp, BUTTON_HEIGHT, BUTTON_WIDTH, GAP_BETWEEN_ITEM, MARGIN,
};
use crate::window::Window;
use conrod_core::widget_ids;
use gluesql::chrono::Utc;
use gluesql::data::Value;

use winit::event::VirtualKeyCode;

widget_ids! {
    pub struct ClassicScoreSceneIds {
        // The main canvas
        canvas,
        title_label,
        see_score_history_button,
        next_button,

        accuracy_canvas,
        accuracy_label,
        accuracy_value_label,

        hit_canvas,
        hit_label,
        hit_value_label,

        miss_canvas,
        miss_label,
        miss_value_label,

        score_canvas,
        score_label,
        score_value_label,

        avg_hit_time_canvas,
        avg_hit_time_label,
        avg_hit_time_value_label,
    }
}

pub struct ClassicGameScoreDisplay {
    pub accuracy: f32,
    pub hit: u16,
    pub miss: u16,
    pub score: i32,
    pub avg_hit_time: f32,
    pub created_at: NaiveDateTime,
}

impl ClassicGameScoreDisplay {
    pub fn new() -> Self {
        Self {
            accuracy: 0.0,
            hit: 0,
            miss: 0,
            score: 0,
            avg_hit_time: 0.0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }

    pub fn read_message(&mut self, message: &MaybeMessage) {
        if let Some(msg) = message {
            self.hit = match *msg.get("hit").unwrap() {
                Value::I64(x) => x,
                _ => unreachable!(),
            } as u16;
            self.miss = match *msg.get("miss").unwrap() {
                Value::I64(x) => x,
                _ => unreachable!(),
            } as u16;
            self.score = match *msg.get("score").unwrap() {
                Value::I64(x) => x,
                _ => unreachable!(),
            } as i32;
            self.created_at = match *msg.get("created_at").unwrap() {
                Value::Timestamp(x) => x,
                _ => unreachable!(),
            };
            self.avg_hit_time = match *msg.get("avg_hit_time").unwrap() {
                Value::F64(x) => x,
                _ => unreachable!(),
            } as f32;
            self.accuracy = self.hit as f32 / (self.hit + self.miss).max(1) as f32 * 100.0;
        }
    }
}

pub struct ClassicScoreScene {
    ids: ClassicScoreSceneIds,
    score: ClassicGameScoreDisplay,
}

impl ClassicScoreScene {
    pub fn new(conrod_handle: &mut ConrodHandle) -> Self {
        let ids = ClassicScoreSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator());
        Self {
            ids,
            score: ClassicGameScoreDisplay::new(),
        }
    }
}

impl Scene for ClassicScoreScene {
    fn init(
        &mut self,
        message: MaybeMessage,
        _window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        database: &mut Database,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = false;

        self.score.read_message(&message);
        database
            .glue
            .execute(&format!(
                "INSERT INTO classic_game_score VALUES (\
                {},\
                {},\
                {},\
                {},\
                {},\
                \"{}\")",
                self.score.hit as f32 / (self.score.hit + self.score.miss).max(1) as f32 * 100.0,
                self.score.hit,
                self.score.miss,
                self.score.score,
                self.score.avg_hit_time,
                Utc::now().naive_utc()
            ))
            .unwrap();
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

        let next_button;
        // let score_history_button;
        {
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

            conrod_core::widget::Canvas::new().set(self.ids.canvas, &mut ui_cell);

            Text::new("Training Report")
                .align_middle_x()
                .mid_top_with_margin_on(self.ids.canvas, MARGIN)
                .set(self.ids.title_label, &mut ui_cell);

            const ITEM_WIDTH: f64 = 400.0;

            conrod_core::widget::Canvas::new()
                .down_from(self.ids.title_label, 50.0)
                .align_middle_x()
                .w(ITEM_WIDTH)
                .set(self.ids.accuracy_canvas, &mut ui_cell);

            conrod_core::widget::Canvas::new()
                .down_from(self.ids.accuracy_canvas, GAP_BETWEEN_ITEM)
                .align_middle_x()
                .w(ITEM_WIDTH)
                .set(self.ids.hit_canvas, &mut ui_cell);

            conrod_core::widget::Canvas::new()
                .down_from(self.ids.hit_canvas, GAP_BETWEEN_ITEM)
                .align_middle_x()
                .w(ITEM_WIDTH)
                .set(self.ids.score_canvas, &mut ui_cell);

            conrod_core::widget::Canvas::new()
                .down_from(self.ids.score_canvas, GAP_BETWEEN_ITEM)
                .align_middle_x()
                .w(ITEM_WIDTH)
                .set(self.ids.avg_hit_time_canvas, &mut ui_cell);

            conrod_core::widget::Canvas::new()
                .down_from(self.ids.avg_hit_time_canvas, GAP_BETWEEN_ITEM)
                .align_middle_x()
                .w(ITEM_WIDTH)
                .set(self.ids.miss_canvas, &mut ui_cell);

            Text::new("Accuracy")
                .mid_left_of(self.ids.accuracy_canvas)
                .left_justify()
                .set(self.ids.accuracy_label, &mut ui_cell);

            Text::new(&format!("{:.1}%", self.score.accuracy))
                .mid_right_of(self.ids.accuracy_canvas)
                .left_justify()
                .set(self.ids.accuracy_value_label, &mut ui_cell);

            Text::new("Hit")
                .mid_left_of(self.ids.hit_canvas)
                .left_justify()
                .set(self.ids.hit_label, &mut ui_cell);

            Text::new(&format!("{}", self.score.hit))
                .mid_right_of(self.ids.hit_canvas)
                .left_justify()
                .set(self.ids.hit_value_label, &mut ui_cell);

            Text::new("Score")
                .mid_left_of(self.ids.score_canvas)
                .left_justify()
                .set(self.ids.score_label, &mut ui_cell);

            Text::new(&format!("{}", self.score.score))
                .mid_right_of(self.ids.score_canvas)
                .left_justify()
                .set(self.ids.score_value_label, &mut ui_cell);

            Text::new("Avg hit time")
                .mid_left_of(self.ids.avg_hit_time_canvas)
                .left_justify()
                .set(self.ids.avg_hit_time_label, &mut ui_cell);

            Text::new(&format!("{:.3}s", self.score.avg_hit_time))
                .mid_right_of(self.ids.avg_hit_time_canvas)
                .left_justify()
                .set(self.ids.avg_hit_time_value_label, &mut ui_cell);

            Text::new("Miss")
                .mid_left_of(self.ids.miss_canvas)
                .left_justify()
                .set(self.ids.miss_label, &mut ui_cell);

            Text::new(&format!("{}", self.score.miss))
                .mid_right_of(self.ids.miss_canvas)
                .left_justify()
                .set(self.ids.miss_value_label, &mut ui_cell);

            next_button = conrod_core::widget::Button::new()
                .label("Next")
                .bottom_right_with_margin_on(self.ids.canvas, MARGIN)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .set(self.ids.next_button, &mut ui_cell);
        }

        if input_manager.is_keyboard_press(&VirtualKeyCode::Escape)
            || input_manager.is_keyboard_press(&VirtualKeyCode::Return)
            || input_manager.is_keyboard_press(&VirtualKeyCode::Space)
            || next_button.was_clicked()
        {
            scene_op = SceneOp::Pop(2, None);
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
