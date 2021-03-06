use crate::audio::AudioContext;

use crate::renderer::Renderer;
use crate::scene::game_score_scene::{
    ClassicGameScoreDisplay, EliminationGameScoreDisplay, HitAndDodgeGameScoreDisplay,
};
use crate::scene::{GameDifficulty, GameMode};

use core::default::Default;
use gluesql::data::Value;
#[cfg(target_arch = "wasm32")]
use gluesql::prelude::MemoryStorage as Storage;
#[cfg(not(target_arch = "wasm32"))]
use gluesql::prelude::SledStorage as Storage;
use gluesql::prelude::{Glue, Payload};
#[cfg(not(target_arch = "wasm32"))]
use gluesql::sled::IVec as Debug;
#[cfg(target_arch = "wasm32")]
use gluesql::storages::memory_storage::Key as Debug;

pub enum GameModeScores {
    Classic(Vec<ClassicGameScoreDisplay>),
    Elimination(Vec<EliminationGameScoreDisplay>),
    HitAndDodge(Vec<HitAndDodgeGameScoreDisplay>),
}

impl Default for GameModeScores {
    fn default() -> Self {
        GameModeScores::Classic(Default::default())
    }
}

impl GameModeScores {
    pub fn read(
        database: &mut Database,
        game_mode: GameMode,
        difficulty: GameDifficulty,
    ) -> GameModeScores {
        match game_mode {
            GameMode::Classic => {
                let output = database
                    .glue
                    .execute(&format!(
                        "SELECT * FROM classic_game_score WHERE difficulty = {} ORDER BY created_at DESC",
                        difficulty as u8
                    ))
                    .unwrap();
                let mut score_rows = Vec::new();
                if let Payload::Select { labels, rows } = output {
                    for row in rows {
                        let mut score = ClassicGameScoreDisplay::new();
                        for (idx, label) in labels.iter().enumerate() {
                            match label.as_str() {
                                "accuracy" => {
                                    score.accuracy = match row[idx] {
                                        Value::F64(x) => x,
                                        _ => unreachable!(),
                                    } as f32;
                                }
                                "hit" => {
                                    score.hit = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as u16;
                                }
                                "miss" => {
                                    score.miss = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as u16;
                                }
                                "score" => {
                                    score.score = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as i32;
                                }
                                "avg_hit_time" => {
                                    score.avg_hit_time = match row[idx] {
                                        Value::F64(x) => x,
                                        _ => unreachable!(),
                                    }
                                        as f32;
                                }
                                "created_at" => {
                                    score.created_at = match row[idx] {
                                        Value::Timestamp(x) => x,
                                        _ => unreachable!(),
                                    };
                                }
                                "difficulty" => {}
                                _ => unreachable!(),
                            }
                        }
                        score_rows.push(score);
                    }
                }
                GameModeScores::Classic(score_rows)
            }
            GameMode::Elimination => {
                let output = database
                    .glue
                    .execute(&format!("SELECT * FROM elimination_game_score WHERE difficulty = {} ORDER BY created_at DESC", difficulty as u8))
                    .unwrap();
                let mut score_rows = Vec::new();
                if let Payload::Select { labels, rows } = output {
                    for row in rows {
                        let mut score = EliminationGameScoreDisplay::new();
                        for (idx, label) in labels.iter().enumerate() {
                            match label.as_str() {
                                "accuracy" => {
                                    score.accuracy = match row[idx] {
                                        Value::F64(x) => x,
                                        _ => unreachable!(),
                                    } as f32;
                                }
                                "hit" => {
                                    score.hit = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as u16;
                                }
                                "miss" => {
                                    score.miss = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as u16;
                                }
                                "score" => {
                                    score.score = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as i32;
                                }
                                "hit_fake_target" => {
                                    score.hit_fake_target = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    }
                                        as u16;
                                }
                                "running_time" => {
                                    score.running_time = match row[idx] {
                                        Value::F64(x) => x,
                                        _ => unreachable!(),
                                    }
                                        as f32;
                                }
                                "avg_hit_time" => {
                                    score.avg_hit_time = match row[idx] {
                                        Value::F64(x) => x,
                                        _ => unreachable!(),
                                    }
                                        as f32;
                                }
                                "created_at" => {
                                    score.created_at = match row[idx] {
                                        Value::Timestamp(x) => x,
                                        _ => unreachable!(),
                                    };
                                }
                                "difficulty" => {}
                                _ => unreachable!(),
                            }
                        }
                        score_rows.push(score);
                    }
                }
                GameModeScores::Elimination(score_rows)
            }
            GameMode::HitAndDodge => {
                let output = database
                    .glue
                    .execute(&format!("SELECT * FROM hit_and_dodge_game_score WHERE difficulty = {} ORDER BY created_at DESC", difficulty as u8))
                    .unwrap();
                let mut score_rows = Vec::new();
                if let Payload::Select { labels, rows } = output {
                    for row in rows {
                        let mut score = HitAndDodgeGameScoreDisplay::new();
                        for (idx, label) in labels.iter().enumerate() {
                            match label.as_str() {
                                "accuracy" => {
                                    score.accuracy = match row[idx] {
                                        Value::F64(x) => x,
                                        _ => unreachable!(),
                                    } as f32;
                                }
                                "hit" => {
                                    score.hit = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as u16;
                                }
                                "miss" => {
                                    score.miss = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as u16;
                                }
                                "score" => {
                                    score.score = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as i32;
                                }
                                "hit_taken" => {
                                    score.hit_taken = match row[idx] {
                                        Value::I64(x) => x,
                                        _ => unreachable!(),
                                    } as u16;
                                }
                                "avg_hit_time" => {
                                    score.avg_hit_time = match row[idx] {
                                        Value::F64(x) => x,
                                        _ => unreachable!(),
                                    }
                                        as f32;
                                }
                                "created_at" => {
                                    score.created_at = match row[idx] {
                                        Value::Timestamp(x) => x,
                                        _ => unreachable!(),
                                    };
                                }
                                "difficulty" => {}
                                _ => unreachable!(),
                            }
                        }
                        score_rows.push(score);
                    }
                }
                GameModeScores::HitAndDodge(score_rows)
            }
        }
    }
}

pub struct Database {
    pub glue: Glue<Debug, Storage>,
}

impl Database {
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let storage = Storage::new("db").unwrap();
        #[cfg(target_arch = "wasm32")]
        let storage = Storage::default();
        Self {
            glue: Glue::new(storage),
        }
    }

    pub fn init(&mut self) {
        self.glue
            .execute(
                "CREATE TABLE IF NOT EXISTS settings (
    audio_volume FLOAT NOT NULL,
    maximum_raymarch_step INTEGER NOT NULL,
    ambient_occlusion_sample INTEGER NOT NULL,

    crosshair_color_r FLOAT NOT NULL,
    crosshair_color_g FLOAT NOT NULL,
    crosshair_color_b FLOAT NOT NULL,

    center_dot_enable BOOLEAN NOT NULL,
    center_dot_thickness FLOAT NOT NULL,

    inner_line_enable BOOLEAN NOT NULL,
    inner_line_thickness FLOAT NOT NULL,
    inner_line_length FLOAT NOT NULL,
    inner_line_offset FLOAT NOT NULL,

    outer_line_enable BOOLEAN NOT NULL,
    outer_line_thickness FLOAT NOT NULL,
    outer_line_length FLOAT NOT NULL,
    outer_line_offset FLOAT NOT NULL,

    mouse_sensitivity FLOAT NOT NULL
)",
            )
            .unwrap();
        self.glue
            .execute(
                "CREATE TABLE IF NOT EXISTS classic_game_score (
    difficulty INTEGER NOT NULL,
    accuracy FLOAT NOT NULL,
    hit INTEGER NOT NULL,
    miss INTEGER NOT NULL,
    score INTEGER NOT NULL,
    avg_hit_time FLOAT NOT NULL,
    created_at TIMESTAMP NOT NULL,
)",
            )
            .unwrap();
        self.glue
            .execute(
                "CREATE TABLE IF NOT EXISTS elimination_game_score (
    difficulty INTEGER NOT NULL,
    accuracy FLOAT NOT NULL,
    hit INTEGER NOT NULL,
    miss INTEGER NOT NULL,
    score INTEGER NOT NULL,
    avg_hit_time FLOAT NOT NULL,
    hit_fake_target INTEGER NOT NULL,
    running_time FLOAT NOT NULL,
    created_at TIMESTAMP NOT NULL,
)",
            )
            .unwrap();
        self.glue
            .execute(
                "CREATE TABLE IF NOT EXISTS hit_and_dodge_game_score (
    difficulty INTEGER NOT NULL,
    accuracy FLOAT NOT NULL,
    hit INTEGER NOT NULL,
    miss INTEGER NOT NULL,
    score INTEGER NOT NULL,
    avg_hit_time FLOAT NOT NULL,
    hit_taken INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
)",
            )
            .unwrap();
        let output = self.glue.execute("SELECT * FROM settings").unwrap();
        if let Payload::Select { rows, .. } = output {
            if rows.is_empty() {
                self.glue
                    .execute(
                        "INSERT INTO settings VALUES (1.0, 50, 3,\
                        1.0, 0.0, 0.0,\
                        TRUE, 2.0,\
                        TRUE, 6.0, 20.0, 5.0,\
                        TRUE, 3.0, 6.0, 49.0,\
                        0.5)",
                    )
                    .unwrap();
            }
        }
    }

    pub fn init_settings(&mut self, audio_context: &mut AudioContext, renderer: &mut Renderer) {
        let output = self.glue.execute("SELECT * FROM settings").unwrap();
        if let Payload::Select { labels, rows } = output {
            for (idx, label) in labels.iter().enumerate() {
                match label.as_str() {
                    "audio_volume" => {
                        audio_context.set_volume(match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32);
                    }
                    "maximum_raymarch_step" => {
                        renderer.rendering_info.queuecount_raymarchmaxstep_aostep.y =
                            match rows[0][idx] {
                                Value::I64(x) => x,
                                _ => unreachable!(),
                            } as u32;
                    }
                    "ambient_occlusion_sample" => {
                        renderer.rendering_info.queuecount_raymarchmaxstep_aostep.z =
                            match rows[0][idx] {
                                Value::I64(x) => x,
                                _ => unreachable!(),
                            } as u32;
                    }
                    "mouse_sensitivity" => {
                        renderer.camera.sensitivity = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "crosshair_color_r" => {
                        renderer.crosshair.color.x = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "crosshair_color_g" => {
                        renderer.crosshair.color.y = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "crosshair_color_b" => {
                        renderer.crosshair.color.z = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "center_dot_enable" => {
                        renderer.crosshair.center_dot_enabled = match rows[0][idx] {
                            Value::Bool(x) => x,
                            _ => unreachable!(),
                        };
                    }
                    "center_dot_thickness" => {
                        renderer.crosshair.center_dot_thickness = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "inner_line_enable" => {
                        renderer.crosshair.center_dot_enabled = match rows[0][idx] {
                            Value::Bool(x) => x,
                            _ => unreachable!(),
                        };
                    }
                    "inner_line_thickness" => {
                        renderer.crosshair.inner_line_thickness = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "inner_line_length" => {
                        renderer.crosshair.inner_line_length = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "inner_line_offset" => {
                        renderer.crosshair.inner_line_offset = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "outer_line_enable" => {
                        renderer.crosshair.center_dot_enabled = match rows[0][idx] {
                            Value::Bool(x) => x,
                            _ => unreachable!(),
                        };
                    }
                    "outer_line_thickness" => {
                        renderer.crosshair.outer_line_thickness = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "outer_line_length" => {
                        renderer.crosshair.outer_line_length = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    "outer_line_offset" => {
                        renderer.crosshair.outer_line_offset = match rows[0][idx] {
                            Value::F64(x) => x,
                            _ => unreachable!(),
                        } as f32;
                    }
                    _ => unreachable!(),
                };
            }
        }
    }
}
