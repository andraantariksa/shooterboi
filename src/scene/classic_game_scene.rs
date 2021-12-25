use conrod_core::{Color, Colorable, Dimensions, Positionable, Sizeable, Widget};
use std::collections::HashMap;

use std::io::{BufReader, Cursor};

use hecs::{Entity, World};
use instant::Instant;
use rapier3d::prelude::*;
use winit::event::{MouseButton, VirtualKeyCode};
use winit::event_loop::ControlFlow;

use crate::animation::InOutAnimation;
use crate::audio::{AudioContext, AUDIO_FILE_SHOOT};
use crate::audio::{Sink, AUDIO_FILE_SHOOTED};
use crate::database::Database;
use crate::entity::target::Target;

use crate::frustum::ObjectBound;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::physics::GamePhysics;

use crate::renderer::render_objects::{MaterialType, ShapeType};
use crate::renderer::rendering_info::BackgroundType;
use crate::renderer::Renderer;
use crate::scene::classic_score_scene::ClassicScoreScene;
use crate::scene::pause_scene::PauseScene;
use crate::scene::{
    GameDifficulty, GameState, MaybeMessage, Message, Scene, SceneOp, FINISHING_DURATION,
    MAX_RAYCAST_DISTANCE, PREPARE_DURATION,
};
use crate::systems::container::{enqueue_container, spawn_container};
use crate::systems::crate_box::{enqueue_crate, spawn_crate};
use crate::systems::player::{init_player, setup_player_collider};
use crate::systems::target::{enqueue_target, spawn_target};
use crate::systems::update_player_movement::update_player_position;
use crate::systems::wall::{enqueue_wall, spawn_wall};
use crate::timer::{Stopwatch, Timer};
use crate::util::lerp;
use crate::window::Window;
use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::widget::{Canvas, Text};
use conrod_core::widget_ids;
use gluesql::data::Value;
use nalgebra::{Vector3, Vector4};
use rand::distributions::Uniform;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

enum TargetSpawnState {
    Primary,
    Secondary,
}

widget_ids! {
    pub struct ClassicGameSceneIds {
        // The main canvas
        canvas,
        canvas_duration,
        duration_label,
        start_duration_label
    }
}

pub struct Score {
    pub hit: u16,
    pub miss: u16,
    pub score: i32,
    pub total_shoot_time: f32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            hit: 0,
            miss: 0,
            score: 0,
            total_shoot_time: 0.0,
        }
    }

    pub fn write_message(&self, message: &mut Message) {
        message.insert("hit", Value::I64(self.hit as i64));
        message.insert("miss", Value::I64(self.miss as i64));
        message.insert("score", Value::I64(self.score as i64));
        message.insert(
            "avg_hit_time",
            Value::F64(self.total_shoot_time as f64 / self.hit.max(1) as f64),
        );
    }
}

pub struct ClassicGameScene {
    ids: ClassicGameSceneIds,
    world: World,
    physics: GamePhysics,
    player_rigid_body_handle: RigidBodyHandle,
    game_state: GameState,
    delta_shoot_time: Stopwatch,
    shoot_timer: Timer,
    score: Score,
    rng: SmallRng,
    round_timer: Timer,
    shoot_animation: InOutAnimation,
    target_spawn_state: TargetSpawnState,
    secondary_delete_duration: Timer,
    entity_to_remove: Vec<Entity>,
    freeze: bool,
    difficulty: GameDifficulty,
}

impl ClassicGameScene {
    pub fn new(
        _renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        difficulty: GameDifficulty,
    ) -> Self {
        let mut world = World::new();
        let mut physics = GamePhysics::new();

        // Ground
        physics
            .collider_set
            .insert(ColliderBuilder::new(SharedShape::cuboid(20.0, 1.0, 10.0)).build());

        let player_rigid_body_handle =
            setup_player_collider(&mut physics, Vector3::new(0.0, 1.0, 0.0));

        spawn_target(
            &mut world,
            &mut physics,
            Vector3::new(0.0, 3.0, -15.0),
            Target::new(None, None),
        );

        spawn_container(
            &mut world,
            &mut physics,
            Vector3::new(20.0 - 1.0 - 2.0, 1.0 + 2.0, -1.0),
            Vector3::new(1.99, 1.99, 4.99),
        );

        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(0.0, 1.4, -9.5),
            Vector3::new(20.0, 0.398, 0.5),
        );
        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(0.0, 1.4, 9.5),
            Vector3::new(20.0, 0.398, 0.5),
        );
        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(19.5, 1.4, 0.0),
            Vector3::new(0.5, 0.398, 9.99),
        );
        spawn_wall(
            &mut world,
            &mut physics,
            Vector3::new(-19.5, 1.4, 0.0),
            Vector3::new(0.5, 0.398, 9.99),
        );

        spawn_crate(
            &mut world,
            &mut physics,
            Vector3::new(-4.0, 4.0, 8.0),
            Vector3::new(0.99, 0.99, 0.99),
        );
        spawn_crate(
            &mut world,
            &mut physics,
            Vector3::new(-6.0, 2.0, 8.0),
            Vector3::new(0.99, 0.99, 0.99),
        );
        spawn_crate(
            &mut world,
            &mut physics,
            Vector3::new(-4.0, 2.0, 8.0),
            Vector3::new(0.99, 0.99, 0.99),
        );
        spawn_crate(
            &mut world,
            &mut physics,
            Vector3::new(-2.0, 2.0, 8.0),
            Vector3::new(0.99, 0.99, 0.99),
        );

        Self {
            world,
            physics,
            player_rigid_body_handle,
            ids: ClassicGameSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
            score: Score::new(),
            delta_shoot_time: Stopwatch::new(),
            game_state: GameState::Preround,
            shoot_timer: Timer::new_finished(),
            target_spawn_state: TargetSpawnState::Primary,
            rng: SmallRng::from_entropy(),
            shoot_animation: InOutAnimation::new(3.0, 5.0),
            secondary_delete_duration: Timer::new(1.0),
            entity_to_remove: Vec::new(),
            round_timer: Timer::new(100.0),
            freeze: false,
            difficulty,
        }
    }
}

impl Scene for ClassicGameScene {
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

        renderer.rendering_info.background_type = BackgroundType::Forest;

        init_player(
            &mut self.physics,
            renderer,
            self.player_rigid_body_handle.clone(),
        );

        // Ground
        let (objects, ref mut bound) = renderer.render_objects.next_static();
        objects.position = nalgebra::Vector3::new(0.0, 0.0, 0.0);
        objects.shape_type_material_ids.0 = ShapeType::Box;
        objects.shape_type_material_ids.1 = MaterialType::CobblestonePaving;
        objects.shape_data1 = nalgebra::Vector4::new(20.0, 1.0, 10.0, 0.0);
        *bound = objects.get_bounding_sphere_radius();

        // Back Wall
        let (objects, ref mut bound) = renderer.render_objects.next_static();
        objects.position = nalgebra::Vector3::new(0.0, 0.0, -40.0);
        objects.shape_type_material_ids.0 = ShapeType::Box;
        objects.shape_type_material_ids.1 = MaterialType::StoneWall;
        objects.shape_data1 = nalgebra::Vector4::new(20.0, 12.0, 1.0, 0.0);
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
        let round_timer_sec = self.round_timer.get_duration();

        let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
        {
            Canvas::new()
                .color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
                .set(self.ids.canvas, &mut ui_cell);

            Canvas::new()
                .color(Color::Rgba(1.0, 1.0, 1.0, 0.3))
                .mid_top_of(self.ids.canvas)
                .wh(Dimensions::new(100.0, 30.0))
                .set(self.ids.canvas_duration, &mut ui_cell);

            Text::new(&format!(
                "{:02}:{:02}",
                (round_timer_sec / 60.0) as i32,
                (round_timer_sec % 60.0) as i32
            ))
            .color(conrod_core::color::BLACK)
            .middle_of(self.ids.canvas_duration)
            .set(self.ids.duration_label, &mut ui_cell);
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
                    self.freeze = false;
                    self.game_state = GameState::Round;
                    renderer.game_renderer.render_crosshair = true;
                }
            }
            GameState::Round => {
                self.round_timer.update(delta_time);
                self.shoot_timer.update(delta_time);
                self.delta_shoot_time.update(delta_time);

                let mut missed = || {
                    self.score.score -= 100;
                    self.score.miss += 1;
                };

                let mut missed_secondary = false;
                for (id, (target, collider_handle)) in
                    self.world.query_mut::<(&mut Target, &ColliderHandle)>()
                {
                    if target.is_need_to_be_deleted(delta_time) {
                        self.entity_to_remove.push(id);
                        self.physics.collider_set.remove(
                            *collider_handle,
                            &mut self.physics.island_manager,
                            &mut self.physics.rigid_body_set,
                            false,
                        );

                        if !target.is_shooted() {
                            missed_secondary = true;
                        }
                    }
                }
                for entity in self.entity_to_remove.iter() {
                    self.world.despawn(*entity).unwrap();
                }
                self.entity_to_remove.clear();

                if missed_secondary {
                    self.target_spawn_state = match self.target_spawn_state {
                        TargetSpawnState::Secondary => TargetSpawnState::Primary,
                        _ => unreachable!(),
                    };
                    missed();
                    spawn_target(
                        &mut self.world,
                        &mut self.physics,
                        Vector3::new(0.0, 3.0, self.rng.sample(Uniform::new(-19.0, -13.0))),
                        Target::new(None, None),
                    );
                }

                self.shoot_animation.update(delta_time);
                renderer.rendering_info.fov_shootanim.y = lerp(
                    0.0f32,
                    -20.0f32.to_radians(),
                    self.shoot_animation.get_value(),
                );

                if input_manager.is_mouse_press(&MouseButton::Left)
                    && self.shoot_timer.is_finished()
                {
                    self.shoot_animation.trigger();
                    self.shoot_timer.reset(0.4);

                    let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
                    sink.append(
                        rodio::Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE_SHOOT.to_vec())))
                            .unwrap(),
                    );
                    audio_context.push(Sink::Regular(sink));

                    let ray = Ray::new(
                        nalgebra::Point::from(
                            renderer.camera.position
                                + renderer.camera.get_direction().into_inner() * 1.0,
                        ),
                        renderer.camera.get_direction().into_inner(),
                    );
                    if let Some((handle, _distance)) = self.physics.query_pipeline.cast_ray(
                        &self.physics.collider_set,
                        &ray,
                        MAX_RAYCAST_DISTANCE,
                        true,
                        self.physics.interaction_groups,
                        None,
                    ) {
                        let collider = self.physics.collider_set.get(handle).unwrap();
                        let entity = Entity::from_bits(collider.user_data as u64);

                        let mut need_to_spawn = false;
                        if let Ok(mut target) = self.world.get_mut::<Target>(entity) {
                            let sink =
                                rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
                            sink.append(
                                rodio::Decoder::new(BufReader::new(Cursor::new(
                                    AUDIO_FILE_SHOOTED.to_vec(),
                                )))
                                .unwrap(),
                            );
                            audio_context.push(Sink::Regular(sink));

                            if !target.is_shooted() {
                                let shoot_time = self.delta_shoot_time.get_duration();
                                self.delta_shoot_time.reset();

                                self.score.total_shoot_time += shoot_time;

                                need_to_spawn = true;
                                target.shooted();

                                self.target_spawn_state = match self.target_spawn_state {
                                    TargetSpawnState::Primary => TargetSpawnState::Secondary,
                                    TargetSpawnState::Secondary => TargetSpawnState::Primary,
                                };

                                self.score.score += ((300.0 * (3.0 - shoot_time)) as i32).max(0);
                                self.score.hit += 1;
                            } else {
                                missed();
                            }
                        } else {
                            missed();
                        }

                        if need_to_spawn {
                            match self.target_spawn_state {
                                TargetSpawnState::Primary => spawn_target(
                                    &mut self.world,
                                    &mut self.physics,
                                    Vector3::new(
                                        0.0,
                                        3.0,
                                        self.rng.sample(Uniform::new(-39.0, -15.0)),
                                    ),
                                    Target::new(None, None),
                                ),
                                TargetSpawnState::Secondary => {
                                    let pos = nalgebra::Vector3::new(
                                        self.rng.sample(Uniform::new(-13.0, 13.0)),
                                        self.rng.sample(Uniform::new(1.0, 5.0)),
                                        self.rng.sample(Uniform::new(-39.0, -18.0)),
                                    );
                                    spawn_target(
                                        &mut self.world,
                                        &mut self.physics,
                                        pos,
                                        Target::new_with_delete_duration(
                                            self.secondary_delete_duration.clone(),
                                            None,
                                            None,
                                        ),
                                    );
                                }
                            }
                        }
                    } else {
                        missed();
                    }
                }

                if self.round_timer.is_finished() {
                    self.game_state = GameState::Finishing(Timer::new(FINISHING_DURATION));
                }
            }
            GameState::Finishing(ref mut timer) => {
                timer.update(delta_time);

                Text::new("Time out!")
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
            scene_op = SceneOp::Push(
                Box::new(ClassicScoreScene::new(conrod_handle)),
                Some({
                    let mut m = HashMap::new();
                    self.score.write_message(&mut m);
                    m
                }),
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
        enqueue_target(&mut self.world, &mut self.physics, renderer);
        enqueue_crate(&mut self.world, &mut self.physics, renderer);
        enqueue_wall(
            &mut self.world,
            &mut self.physics,
            renderer,
            MaterialType::StoneWall,
        );
        enqueue_container(&mut self.world, &mut self.physics, renderer);
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
