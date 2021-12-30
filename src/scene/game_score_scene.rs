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
                    score.hit as f32 / (score.hit + score.miss).max(1) as f32 * 100.0,
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
                \"{}\")",
                    self.difficulty as u8,
                    score.hit as f32 / (score.hit + score.miss).max(1) as f32 * 100.0,
                    score.hit,
                    score.miss,
                    score.score,
                    score.avg_hit_time,
                    score.hit_fake_target,
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
                    score.hit as f32 / (score.hit + score.miss).max(1) as f32 * 100.0,
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
                    Canvas::new()
                        .down_from(self.ids.title_label, 50.0)
                        .align_middle_x()
                        .w(ITEM_WIDTH)
                        .set(self.ids.accuracy_canvas, &mut ui_cell);

                    Canvas::new()
                        .down_from(self.ids.accuracy_canvas, GAP_BETWEEN_ITEM)
                        .align_middle_x()
                        .w(ITEM_WIDTH)
                        .set(self.ids.hit_canvas, &mut ui_cell);

                    Canvas::new()
                        .down_from(self.ids.hit_canvas, GAP_BETWEEN_ITEM)
                        .align_middle_x()
                        .w(ITEM_WIDTH)
                        .set(self.ids.score_canvas, &mut ui_cell);

                    Canvas::new()
                        .down_from(self.ids.score_canvas, GAP_BETWEEN_ITEM)
                        .align_middle_x()
                        .w(ITEM_WIDTH)
                        .set(self.ids.avg_hit_time_canvas, &mut ui_cell);

                    Canvas::new()
                        .down_from(self.ids.avg_hit_time_canvas, GAP_BETWEEN_ITEM)
                        .align_middle_x()
                        .w(ITEM_WIDTH)
                        .set(self.ids.miss_canvas, &mut ui_cell);

                    Text::new("Accuracy")
                        .mid_left_of(self.ids.accuracy_canvas)
                        .left_justify()
                        .set(self.ids.accuracy_label, &mut ui_cell);

                    Text::new(&format!("{:.1}%", score.accuracy))
                        .mid_right_of(self.ids.accuracy_canvas)
                        .left_justify()
                        .set(self.ids.accuracy_value_label, &mut ui_cell);

                    Text::new("Hit")
                        .mid_left_of(self.ids.hit_canvas)
                        .left_justify()
                        .set(self.ids.hit_label, &mut ui_cell);

                    Text::new(&format!("{}", score.hit))
                        .mid_right_of(self.ids.hit_canvas)
                        .left_justify()
                        .set(self.ids.hit_value_label, &mut ui_cell);

                    Text::new("Score")
                        .mid_left_of(self.ids.score_canvas)
                        .left_justify()
                        .set(self.ids.score_label, &mut ui_cell);

                    Text::new(&format!("{}", score.score))
                        .mid_right_of(self.ids.score_canvas)
                        .left_justify()
                        .set(self.ids.score_value_label, &mut ui_cell);

                    Text::new("Avg hit time")
                        .mid_left_of(self.ids.avg_hit_time_canvas)
                        .left_justify()
                        .set(self.ids.avg_hit_time_label, &mut ui_cell);

                    Text::new(&format!("{:.3}s", score.avg_hit_time))
                        .mid_right_of(self.ids.avg_hit_time_canvas)
                        .left_justify()
                        .set(self.ids.avg_hit_time_value_label, &mut ui_cell);

                    Text::new("Miss")
                        .mid_left_of(self.ids.miss_canvas)
                        .left_justify()
                        .set(self.ids.miss_label, &mut ui_cell);

                    Text::new(&format!("{}", score.miss))
                        .mid_right_of(self.ids.miss_canvas)
                        .left_justify()
                        .set(self.ids.miss_value_label, &mut ui_cell);
                }
                GameModeScore::Elimination(_) => {}
                GameModeScore::HitAndDodge(_) => {}
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
