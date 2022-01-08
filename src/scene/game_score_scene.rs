use chrono::NaiveDateTime;
use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::widget::{Button, Canvas, Text};
use conrod_core::{Labelable, Positionable, Sizeable, Widget};

use winit::event_loop::ControlFlow;

use crate::audio::AudioContext;
use crate::database::Database;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;

use crate::scene::{
    GameDifficulty, MaybeMessage, Scene, SceneOp, BUTTON_HEIGHT, BUTTON_WIDTH, GAP_BETWEEN_ITEM,
    MARGIN,
};
use crate::window::Window;
use conrod_core::widget_ids;

use gluesql::chrono::Utc;

use crate::scene::score_history_scene::ScoreHistoryScene;
use winit::event::VirtualKeyCode;

#[derive(Debug)]
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
            created_at: Utc::now().naive_utc(),
        }
    }
}

pub struct EliminationGameScoreDisplay {
    pub accuracy: f32,
    pub hit: u16,
    pub miss: u16,
    pub hit_fake_target: u16,
    pub score: i32,
    pub avg_hit_time: f32,
    pub running_time: f32,
    pub created_at: NaiveDateTime,
}

impl EliminationGameScoreDisplay {
    pub fn new() -> Self {
        Self {
            accuracy: 0.0,
            hit: 0,
            miss: 0,
            score: 0,
            avg_hit_time: 0.0,
            hit_fake_target: 0,
            running_time: 0.0,
            created_at: Utc::now().naive_utc(),
        }
    }
}

pub struct HitAndDodgeGameScoreDisplay {
    pub accuracy: f32,
    pub hit: u16,
    pub miss: u16,
    pub score: i32,
    pub hit_taken: u16,
    pub avg_hit_time: f32,
    pub created_at: NaiveDateTime,
}

impl HitAndDodgeGameScoreDisplay {
    pub fn new() -> Self {
        Self {
            accuracy: 0.0,
            hit: 0,
            miss: 0,
            score: 0,
            avg_hit_time: 0.0,
            hit_taken: 0,
            created_at: Utc::now().naive_utc(),
        }
    }
}

pub enum GameModeScore {
    Classic(ClassicGameScoreDisplay),
    Elimination(EliminationGameScoreDisplay),
    HitAndDodge(HitAndDodgeGameScoreDisplay),
}

pub struct GameScoreScene {
    ids: GameScoreSceneIds,
    score: GameModeScore,
    difficulty: GameDifficulty,
}

widget_ids! {
    pub struct GameScoreSceneIds {
        // The main canvas
        canvas,
        title_label,
        score_history_button,
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

        hit_taken_canvas,
        hit_taken_label,
        hit_taken_value_label,

        hit_fake_target_canvas,
        hit_fake_target_label,
        hit_fake_target_value_label,

        running_time_canvas,
        running_time_label,
        running_time_value_label,
    }
}

impl GameScoreScene {
    pub fn new(
        conrod_handle: &mut ConrodHandle,
        score: GameModeScore,
        difficulty: GameDifficulty,
    ) -> Self {
        let ids = GameScoreSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator());
        Self {
            ids,
            score,
            difficulty,
        }
    }
}

impl Scene for GameScoreScene {
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

        let query = match &self.score {
            GameModeScore::Classic(score) => {
                format!(
                    "INSERT INTO classic_game_score VALUES (\
                {},\
                {},\
                {},\
                {},\
                {},\
                {},\
                \"{}\")",
                    self.difficulty as u8,
                    score.accuracy,
                    score.hit,
                    score.miss,
                    score.score,
                    score.avg_hit_time,
                    score.created_at
                )
            }
            GameModeScore::Elimination(score) => {
                format!(
                    "INSERT INTO elimination_game_score VALUES (\
                {},\
                {},\
                {},\
                {},\
                {},\
                {},\
                {},\
                {},\
                \"{}\")",
                    self.difficulty as u8,
                    score.accuracy,
                    score.hit,
                    score.miss,
                    score.score,
                    score.avg_hit_time,
                    score.hit_fake_target,
                    score.running_time,
                    Utc::now().naive_utc()
                )
            }
            GameModeScore::HitAndDodge(score) => {
                format!(
                    "INSERT INTO hit_and_dodge_game_score VALUES (\
                {},\
                {},\
                {},\
                {},\
                {},\
                {},\
                {},\
                \"{}\")",
                    self.difficulty as u8,
                    score.accuracy,
                    score.hit,
                    score.miss,
                    score.score,
                    score.avg_hit_time,
                    score.hit_taken,
                    Utc::now().naive_utc()
                )
            }
        };
        database.glue.execute(&query).unwrap();
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
        let score_history_button;
        {
            let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();

            Canvas::new().set(self.ids.canvas, &mut ui_cell);

            Text::new("Training Report")
                .align_middle_x()
                .mid_top_with_margin_on(self.ids.canvas, MARGIN)
                .set(self.ids.title_label, &mut ui_cell);

            const ITEM_WIDTH: f64 = 400.0;

            match &self.score {
                GameModeScore::Classic(score) => {
                    let mut prev_canvas_id = self.ids.title_label;

                    let l = [
                        (
                            &self.ids.hit_canvas,
                            &self.ids.hit_label,
                            "Hit",
                            &self.ids.hit_value_label,
                            &format!("{}", score.hit),
                        ),
                        (
                            &self.ids.score_canvas,
                            &self.ids.score_label,
                            "Score",
                            &self.ids.score_value_label,
                            &format!("{}", score.score),
                        ),
                        (
                            &self.ids.avg_hit_time_canvas,
                            &self.ids.avg_hit_time_label,
                            "Avg hit time",
                            &self.ids.avg_hit_time_value_label,
                            &format!("{:.2}s", score.avg_hit_time),
                        ),
                        (
                            &self.ids.miss_canvas,
                            &self.ids.miss_label,
                            "Miss",
                            &self.ids.miss_value_label,
                            &format!("{}", score.miss),
                        ),
                        (
                            &self.ids.accuracy_canvas,
                            &self.ids.accuracy_label,
                            "Accuracy",
                            &self.ids.accuracy_value_label,
                            &format!("{:.2}%", score.accuracy),
                        ),
                    ];

                    for (i, (id_canvas, id_label, label_text, id_value_label, value_text)) in
                        l.iter().enumerate()
                    {
                        Canvas::new()
                            .down_from(prev_canvas_id, if i == 0 { 50.0 } else { GAP_BETWEEN_ITEM })
                            .align_middle_x()
                            .w(ITEM_WIDTH)
                            .set(**id_canvas, &mut ui_cell);

                        prev_canvas_id = **id_canvas;
                    }
                    for (id_canvas, id_label, label_text, id_value_label, value_text) in l.iter() {
                        Text::new(label_text)
                            .mid_left_of(**id_canvas)
                            .left_justify()
                            .set(**id_label, &mut ui_cell);

                        Text::new(value_text)
                            .mid_right_of(**id_canvas)
                            .left_justify()
                            .set(**id_value_label, &mut ui_cell);
                    }
                }
                GameModeScore::Elimination(score) => {
                    let mut prev_canvas_id = self.ids.title_label;

                    let l = [
                        (
                            &self.ids.hit_canvas,
                            &self.ids.hit_label,
                            "Hit",
                            &self.ids.hit_value_label,
                            &format!("{}", score.hit),
                        ),
                        (
                            &self.ids.score_canvas,
                            &self.ids.score_label,
                            "Score",
                            &self.ids.score_value_label,
                            &format!("{}", score.score),
                        ),
                        (
                            &self.ids.avg_hit_time_canvas,
                            &self.ids.avg_hit_time_label,
                            "Avg hit time",
                            &self.ids.avg_hit_time_value_label,
                            &format!("{:.2}s", score.avg_hit_time),
                        ),
                        (
                            &self.ids.miss_canvas,
                            &self.ids.miss_label,
                            "Miss",
                            &self.ids.miss_value_label,
                            &format!("{}", score.miss),
                        ),
                        (
                            &self.ids.accuracy_canvas,
                            &self.ids.accuracy_label,
                            "Accuracy",
                            &self.ids.accuracy_value_label,
                            &format!("{:.2}%", score.accuracy),
                        ),
                        (
                            &self.ids.hit_fake_target_canvas,
                            &self.ids.hit_fake_target_label,
                            "Hit fake target",
                            &self.ids.hit_fake_target_value_label,
                            &format!("{}", score.hit_fake_target),
                        ),
                        (
                            &self.ids.running_time_canvas,
                            &self.ids.running_time_label,
                            "Running time",
                            &self.ids.running_time_value_label,
                            &format!(
                                "{:02}:{:02}",
                                (score.running_time / 60.0) as i32,
                                (score.running_time % 60.0) as i32
                            ),
                        ),
                    ];

                    for (i, (id_canvas, id_label, label_text, id_value_label, value_text)) in
                        l.iter().enumerate()
                    {
                        Canvas::new()
                            .down_from(prev_canvas_id, if i == 0 { 50.0 } else { GAP_BETWEEN_ITEM })
                            .align_middle_x()
                            .w(ITEM_WIDTH)
                            .set(**id_canvas, &mut ui_cell);

                        prev_canvas_id = **id_canvas;
                    }
                    for (id_canvas, id_label, label_text, id_value_label, value_text) in l.iter() {
                        Text::new(label_text)
                            .mid_left_of(**id_canvas)
                            .left_justify()
                            .set(**id_label, &mut ui_cell);

                        Text::new(value_text)
                            .mid_right_of(**id_canvas)
                            .left_justify()
                            .set(**id_value_label, &mut ui_cell);
                    }
                }
                GameModeScore::HitAndDodge(score) => {
                    let mut prev_canvas_id = self.ids.title_label;

                    let l = [
                        (
                            &self.ids.hit_canvas,
                            &self.ids.hit_label,
                            "Hit",
                            &self.ids.hit_value_label,
                            &format!("{}", score.hit),
                        ),
                        (
                            &self.ids.score_canvas,
                            &self.ids.score_label,
                            "Score",
                            &self.ids.score_value_label,
                            &format!("{}", score.score),
                        ),
                        (
                            &self.ids.avg_hit_time_canvas,
                            &self.ids.avg_hit_time_label,
                            "Avg hit time",
                            &self.ids.avg_hit_time_value_label,
                            &format!("{:.2}s", score.avg_hit_time),
                        ),
                        (
                            &self.ids.miss_canvas,
                            &self.ids.miss_label,
                            "Miss",
                            &self.ids.miss_value_label,
                            &format!("{}", score.miss),
                        ),
                        (
                            &self.ids.accuracy_canvas,
                            &self.ids.accuracy_label,
                            "Accuracy",
                            &self.ids.accuracy_value_label,
                            &format!("{:.2}%", score.accuracy),
                        ),
                        (
                            &self.ids.hit_taken_canvas,
                            &self.ids.hit_taken_label,
                            "Hit taken",
                            &self.ids.hit_taken_value_label,
                            &format!("{}", score.hit_taken),
                        ),
                    ];

                    for (i, (id_canvas, id_label, label_text, id_value_label, value_text)) in
                        l.iter().enumerate()
                    {
                        Canvas::new()
                            .down_from(prev_canvas_id, if i == 0 { 50.0 } else { GAP_BETWEEN_ITEM })
                            .align_middle_x()
                            .w(ITEM_WIDTH)
                            .set(**id_canvas, &mut ui_cell);

                        prev_canvas_id = **id_canvas;
                    }
                    for (id_canvas, id_label, label_text, id_value_label, value_text) in l.iter() {
                        Text::new(label_text)
                            .mid_left_of(**id_canvas)
                            .left_justify()
                            .set(**id_label, &mut ui_cell);

                        Text::new(value_text)
                            .mid_right_of(**id_canvas)
                            .left_justify()
                            .set(**id_value_label, &mut ui_cell);
                    }
                }
            }

            next_button = Button::new()
                .label("Next")
                .bottom_right_with_margin_on(self.ids.canvas, MARGIN)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .set(self.ids.next_button, &mut ui_cell);

            score_history_button = Button::new()
                .label("Past Score")
                .up_from(self.ids.next_button, GAP_BETWEEN_ITEM)
                .wh(conrod_core::Dimensions::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                .set(self.ids.score_history_button, &mut ui_cell);
        }

        if score_history_button.was_clicked() {
            scene_op = SceneOp::Replace(
                Box::new(ScoreHistoryScene::new(_renderer, conrod_handle)),
                None,
            );
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
