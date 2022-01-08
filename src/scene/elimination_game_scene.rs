use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{color, Color, Colorable, Positionable, Sizeable, Widget};

use chrono::Utc;
use std::io::{BufReader, Cursor};

use hecs::{Entity, World};

use rapier3d::prelude::*;
use winit::event::{MouseButton, VirtualKeyCode};
use winit::event_loop::ControlFlow;

use crate::animation::InOutAnimation;
use crate::audio::{AudioContext, AUDIO_FILE_SHOOT};
use crate::audio::{Sink, AUDIO_FILE_SHOOTED};
use crate::database::Database;

use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::MaterialType;
use crate::renderer::render_objects::ShapeType;
use crate::renderer::Renderer;
use crate::scene::pause_scene::PauseScene;
use crate::scene::{
    GameDifficulty, GameState, MaybeMessage, Scene, SceneOp, FINISHING_DURATION,
    IN_SHOOT_ANIM_DURATION, OUT_SHOOT_ANIM_DURATION, PREPARE_DURATION,
};
use crate::timer::{Stopwatch, Timer};
use crate::util::lerp;
use crate::window::Window;
use conrod_core::widget::{Canvas, Text};
use conrod_core::widget_ids;

use crate::entity::target::{Patrol, Target, Validity};

use crate::camera::Camera;
use crate::renderer::rendering_info::BackgroundType;
use crate::scene::game_score_scene::{EliminationGameScoreDisplay, GameModeScore, GameScoreScene};
use crate::systems::player::{init_player, setup_player_collider};
use crate::systems::shoot_ray::shoot_ray;
use crate::systems::shootanim::shootanim;
use crate::systems::target::{enqueue_target, is_any_target_exists, spawn_target, update_target};
use crate::systems::update_player_movement::update_player_position;
use crate::systems::wall::{enqueue_wall, spawn_wall};
use nalgebra::Vector3;
use rand::distributions::Uniform;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

widget_ids! {
    pub struct EliminationGameSceneIds {
        // The main canvas
        canvas,
        start_duration_label,

        indicator_canvas,

        duration_canvas,
        duration_label,
        score_canvas,
        score_label,
        accuracy_canvas,
        accuracy_label,
    }
}

pub struct Score {
    pub hit: u16,
    pub miss: u16,
    pub score: i32,
    pub hit_fake_target: u16,
}

impl Score {
    pub fn new() -> Self {
        Self {
            hit: 0,
            miss: 0,
            score: 0,
            hit_fake_target: 0,
        }
    }
}

pub struct EliminationGameScene {
    ids: EliminationGameSceneIds,
    world: World,
    physics: GamePhysics,
    player_rigid_body_handle: RigidBodyHandle,
    delta_shoot_time: Stopwatch,
    shoot_timer: Timer,
    score: Score,
    rng: SmallRng,
    shoot_animation: InOutAnimation,
    entity_to_remove: Vec<Entity>,
    round_stopwatch: Stopwatch,
    freeze: bool,
    game_state: GameState,
    difficulty: GameDifficulty,
}

impl EliminationGameScene {
    pub fn new(
        _renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        difficulty: GameDifficulty,
    ) -> Self {
        let mut world = World::new();
        let mut physics = GamePhysics::new();

        // Ground
        let ground_rigid_body_handle = physics.rigid_body_set.insert(
            RigidBodyBuilder::new(RigidBodyType::Static)
                .translation(Vector3::new(0.0, 35.0, 0.0))
                .build(),
        );
        physics.collider_set.insert_with_parent(
            ColliderBuilder::new(SharedShape::cuboid(3.3, 35.0, 3.3)).build(),
            ground_rigid_body_handle,
            &mut physics.rigid_body_set,
        );

        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(0.0, 70.4, -3.0),
            Vector3::new(3.3, 0.4, 0.3),
        );
        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(0.0, 70.4, 3.0),
            Vector3::new(3.3, 0.4, 0.5),
        );
        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(-3.0, 70.4, 0.0),
            Vector3::new(0.5, 0.4, 3.3),
        );
        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(3.0, 70.4, 0.0),
            Vector3::new(0.5, 0.4, 3.3),
        );

        // Player
        let player_rigid_body_handle =
            setup_player_collider(&mut physics, Vector3::new(0.0, 71.0, 0.0));

        Self {
            world,
            physics,
            player_rigid_body_handle,
            ids: EliminationGameSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
            score: Score::new(),
            delta_shoot_time: Stopwatch::new(),
            shoot_timer: Timer::new_finished(),
            rng: SmallRng::from_entropy(),
            shoot_animation: InOutAnimation::new(IN_SHOOT_ANIM_DURATION, OUT_SHOOT_ANIM_DURATION),
            freeze: false,
            round_stopwatch: Stopwatch::new(),
            entity_to_remove: Vec::new(),
            game_state: GameState::Preround,
            difficulty,
        }
    }
}

impl Scene for EliminationGameScene {
    fn init(
        &mut self,
        message: MaybeMessage,
        window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        _database: &mut Database,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = true;

        renderer.rendering_info.background_type = BackgroundType::City;

        init_player(&mut self.physics, renderer, self.player_rigid_body_handle);

        // Ground
        let (objects, ref mut bound) = renderer.render_objects.next_static();
        objects.position = nalgebra::Vector3::new(0.0, 35.0, 0.0);
        objects.shape_type_material_ids.0 = ShapeType::Box;
        objects.shape_type_material_ids.1 = MaterialType::Asphalt;
        objects.shape_data1 = nalgebra::Vector4::new(3.0, 35.0, 3.0, 0.0);
        *bound = objects.get_bounding_sphere_radius();

        window.set_is_cursor_grabbed(true);

        audio_context.global_sinks_map.remove("bgm");

        if let Some(m) = message {
            if m.contains_key("from_pause") {
                self.freeze = true;
                renderer.game_renderer.render_crosshair = false;
                self.game_state = GameState::Prepare(Timer::new(PREPARE_DURATION))
            }
        }
    }

    fn update(
        &mut self,
        _window: &mut Window,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        _control_flow: &mut ControlFlow,
        _database: &mut Database,
    ) -> SceneOp {
        let round_timer_sec = self.round_stopwatch.get_duration();

        let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
        {
            Canvas::new()
                .color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
                .set(self.ids.canvas, &mut ui_cell);

            Canvas::new()
                .color(Color::Rgba(1.0, 1.0, 1.0, 0.3))
                .mid_top_of(self.ids.canvas)
                .flow_right(&[
                    (
                        self.ids.score_canvas,
                        Canvas::new()
                            .length_weight(0.3)
                            .color(Color::Rgba(1.0, 1.0, 1.0, 0.2)),
                    ),
                    (
                        self.ids.duration_canvas,
                        Canvas::new()
                            .length_weight(0.4)
                            .color(Color::Rgba(1.0, 1.0, 1.0, 0.4)),
                    ),
                    (
                        self.ids.accuracy_canvas,
                        Canvas::new()
                            .length_weight(0.3)
                            .color(Color::Rgba(1.0, 1.0, 1.0, 0.2)),
                    ),
                ])
                .wh(conrod_core::Dimensions::new(200.0, 30.0))
                .set(self.ids.indicator_canvas, &mut ui_cell);

            Text::new(&format!(
                "{:02}:{:02}",
                (round_timer_sec / 60.0) as i32,
                (round_timer_sec % 60.0) as i32
            ))
            .color(color::BLACK)
            .middle_of(self.ids.duration_canvas)
            .set(self.ids.duration_label, &mut ui_cell);

            let _w_duration_label = ui_cell.w_of(self.ids.duration_label).unwrap();

            Text::new(&format!("{}", self.score.score))
                .font_size(12)
                .color(color::BLACK)
                .middle_of(self.ids.score_canvas)
                .set(self.ids.score_label, &mut ui_cell);
            Text::new(&format!(
                "{:.2}%",
                (self.score.hit) as f32 / (self.score.hit + self.score.miss).max(1) as f32 * 100.0
            ))
            .font_size(12)
            .color(color::BLACK)
            .middle_of(self.ids.accuracy_canvas)
            .set(self.ids.accuracy_label, &mut ui_cell);
        }

        let mut game_finished = false;

        let mut scene_op = SceneOp::None;

        if !self.freeze {
            self.physics.physics_pipeline.step(
                &self.physics.gravity,
                &self.physics.integration_parameters,
                &mut self.physics.island_manager,
                &mut self.physics.broad_phase,
                &mut self.physics.narrow_phase,
                &mut self.physics.rigid_body_set,
                &mut self.physics.collider_set,
                &mut self.physics.joint_set,
                &mut self.physics.ccd_solver,
                &(),
                &(),
            );
            self.physics.query_pipeline.update(
                &self.physics.island_manager,
                &self.physics.rigid_body_set,
                &self.physics.collider_set,
            );

            renderer.camera.move_direction(input_manager.mouse_movement);

            let _player_position = update_player_position(
                delta_time,
                input_manager,
                &mut renderer.camera,
                &mut self.physics,
                self.player_rigid_body_handle,
            );
        }

        match self.game_state {
            GameState::Preround => {
                Text::new("Press any mouse key to start")
                    .align_middle_x_of(self.ids.canvas)
                    .align_middle_y_of(self.ids.canvas)
                    .set(self.ids.start_duration_label, &mut ui_cell);

                if input_manager.is_any_mouse_press() {
                    self.game_state = GameState::Prepare(Timer::new(PREPARE_DURATION));
                }
            }
            GameState::Prepare(ref mut timer) => {
                timer.update(delta_time);

                Text::new(&format!("{:.1}", timer.get_duration()))
                    .align_middle_x_of(self.ids.canvas)
                    .align_middle_y_of(self.ids.canvas)
                    .set(self.ids.start_duration_label, &mut ui_cell);

                if timer.is_finished() {
                    if !self.freeze {
                        self.init_targets();
                    }
                    self.freeze = false;
                    self.game_state = GameState::Round;
                    renderer.game_renderer.render_crosshair = true;
                }
            }
            GameState::Round => {
                self.round_stopwatch.update(delta_time);
                self.shoot_timer.update(delta_time);
                self.delta_shoot_time.update(delta_time);

                update_target(
                    &mut self.world,
                    &mut self.physics,
                    delta_time,
                    &mut self.rng,
                );

                for (id, (target, collider_handle)) in
                    self.world.query_mut::<(&mut Target, &ColliderHandle)>()
                {
                    if target.is_need_to_be_deleted() {
                        self.entity_to_remove.push(id);
                        self.physics.collider_set.remove(
                            *collider_handle,
                            &mut self.physics.island_manager,
                            &mut self.physics.rigid_body_set,
                            false,
                        );
                    }
                }
                for entity in self.entity_to_remove.iter() {
                    self.world.despawn(*entity).unwrap();
                }
                self.entity_to_remove.clear();

                shootanim(
                    &mut self.shoot_animation,
                    &mut renderer.rendering_info,
                    delta_time,
                );

                self.shoot(&input_manager, audio_context, &renderer.camera, delta_time);

                if !is_any_target_exists(&mut self.world) {
                    self.game_state = GameState::Finishing(Timer::new(FINISHING_DURATION));
                }
            }
            GameState::Finishing(ref mut timer) => {
                renderer.game_renderer.render_crosshair = false;
                timer.update(delta_time);

                Text::new("Finished!")
                    .align_middle_x_of(self.ids.canvas)
                    .align_middle_y_of(self.ids.canvas)
                    .set(self.ids.start_duration_label, &mut ui_cell);

                if timer.is_finished() {
                    game_finished = true;
                }
            }
        };

        drop(ui_cell);

        if game_finished {
            scene_op = SceneOp::Replace(
                Box::new(GameScoreScene::new(
                    conrod_handle,
                    GameModeScore::Elimination(EliminationGameScoreDisplay {
                        accuracy: self.score.hit as f32
                            / (self.score.hit + self.score.miss).max(1) as f32
                            * 100.0,
                        hit: self.score.hit,
                        miss: self.score.miss,
                        hit_fake_target: self.score.hit_fake_target,
                        score: self.score.score,
                        avg_hit_time: self.round_stopwatch.get_duration()
                            / self.score.hit.max(1) as f32,
                        running_time: self.round_stopwatch.get_duration(),
                        created_at: Utc::now().naive_utc(),
                    }),
                    self.difficulty,
                )),
                None,
            );
        }

        if input_manager.is_keyboard_press(&VirtualKeyCode::Escape) {
            scene_op = SceneOp::Push(Box::new(PauseScene::new(renderer, conrod_handle)), None);
        }

        scene_op
    }

    fn prerender(
        &mut self,
        renderer: &mut Renderer,
        _input_manager: &InputManager,
        _delta_time: f32,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
    ) {
        enqueue_wall(
            &mut self.world,
            &mut self.physics,
            renderer,
            MaterialType::Black,
        );
        enqueue_target(&mut self.world, &mut self.physics, renderer);
    }

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
        _database: &mut Database,
    ) {
        renderer.rendering_info.fov_shootanim.y = 0.0;
        renderer.render_objects.clear();
        window.set_is_cursor_grabbed(false);
    }
}

impl EliminationGameScene {
    fn init_targets(&mut self) {
        let y_amount = match self.difficulty {
            GameDifficulty::Easy => 7,
            GameDifficulty::Medium => 8,
            GameDifficulty::Hard => 9,
        };
        for y in 0..y_amount {
            for _ in 0..2 {
                match self.difficulty {
                    GameDifficulty::Easy => {
                        let r = self.rng.sample(Uniform::new(8.0, 20.0));
                        let angle = self
                            .rng
                            .sample(Uniform::new(-std::f32::consts::PI, std::f32::consts::PI));
                        let pos = Vector3::new(r * angle.cos(), (y + 71) as f32, r * angle.sin());

                        spawn_target(
                            &mut self.world,
                            &mut self.physics,
                            pos,
                            Target::new(None, Patrol::None),
                        );
                    }
                    GameDifficulty::Medium => {
                        let r = self.rng.sample(Uniform::new(8.0, 20.0));
                        let angle = self
                            .rng
                            .sample(Uniform::new(-std::f32::consts::PI, std::f32::consts::PI));
                        let pos = Vector3::new(r * angle.cos(), (y + 71) as f32, r * angle.sin());
                        // 50
                        let valid_random = self.rng.sample(Uniform::new(0, 100));
                        let validity = if valid_random % 2 == 0 {
                            Some(Validity {
                                valid_duration: self.rng.sample(Uniform::new(4.0, 10.0)),
                                invalid_duration: self.rng.sample(Uniform::new(2.0, 5.0)),
                            })
                        } else {
                            None
                        };

                        spawn_target(
                            &mut self.world,
                            &mut self.physics,
                            pos,
                            Target::new(validity, Patrol::None),
                        );
                    }
                    GameDifficulty::Hard => {
                        let r = self.rng.sample(Uniform::new(8.0, 20.0));
                        let angle = self
                            .rng
                            .sample(Uniform::new(-std::f32::consts::PI, std::f32::consts::PI));
                        let angle2 = self
                            .rng
                            .sample(Uniform::new(-std::f32::consts::PI, std::f32::consts::PI));
                        let pos = Vector3::new(r * angle.cos(), (y + 71) as f32, r * angle.sin());

                        spawn_target(
                            &mut self.world,
                            &mut self.physics,
                            pos,
                            Target::new(
                                None,
                                Patrol::Polar {
                                    or: Vector3::new(0.0, 0.0, 0.0),
                                    r,
                                    a: angle,
                                    b: angle2,
                                    c: angle,
                                },
                            ),
                        );
                    }
                };
            }
        }
    }

    fn shoot(
        &mut self,
        input_manager: &InputManager,
        audio_context: &mut AudioContext,
        camera: &Camera,
        delta_time: f32,
    ) {
        if input_manager.is_mouse_press(&MouseButton::Left) && self.shoot_timer.is_finished() {
            self.shoot_animation.trigger();
            self.shoot_timer.reset(0.4);

            let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
            sink.append(
                rodio::Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE_SHOOT.to_vec())))
                    .unwrap(),
            );
            audio_context.push(Sink::Regular(sink));

            if let Some((handle, _distance)) = shoot_ray(&self.physics, camera) {
                let collider = self.physics.collider_set.get(handle).unwrap();
                let entity = Entity::from_bits(collider.user_data as u64).unwrap();

                if let Ok(mut target) = self.world.get_mut::<Target>(entity) {
                    if target.try_shoot(audio_context) {
                        let shoot_time = self.delta_shoot_time.get_duration();
                        self.delta_shoot_time.reset();

                        self.score.hit += 1;
                        self.score.score += ((100.0 * (4.0 - shoot_time)) as i32).max(50);
                    } else {
                        self.score.miss += 1;
                    }
                } else {
                    self.score.miss += 1;
                }
            } else {
                self.score.miss += 1;
            }
        }
    }
}
