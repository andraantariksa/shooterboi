use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Colorable, Positionable, Sizeable, Widget};
use std::collections::HashMap;

use std::io::{BufReader, Cursor};

use hecs::{Entity, World};
use instant::Duration;
use rapier3d::prelude::*;
use winit::event::{MouseButton, VirtualKeyCode};
use winit::event_loop::ControlFlow;

use crate::audio::{AudioContext, AUDIO_FILE_SHOOT};
use crate::camera::ObjectBound;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::physics::GamePhysics;
use crate::renderer::{Renderer, ShapeType};
use crate::scene::classic_score_scene::ClassicScoreScene;
use crate::scene::pause_scene::PauseScene;
use crate::scene::{MaybeMessage, Message, Scene, SceneOp, Value};
use crate::timer::Timer;
use crate::window::Window;
use conrod_core::widget_ids;
use rand::distributions::Uniform;
use rand::prelude::SmallRng;
use rand::{Rng, RngCore, SeedableRng};

#[derive(Debug)]
struct Label(&'static str);

pub struct Target;

pub struct Position(nalgebra::Vector3<f32>);

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
    pub accuracy: f32,
    pub hit: u16,
    pub miss: u16,
    pub score: i32,
    pub avg_hit_time: f32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            hit: 0,
            miss: 0,
            score: 0,
            avg_hit_time: 0.0,
            accuracy: 0.0,
        }
    }

    pub fn read_message(&mut self, message: &MaybeMessage) -> Option<()> {
        if let Some(message) = message {
            self.hit = *message.get("hit")?.to_i32() as u16;
            self.miss = *message.get("miss")?.to_i32() as u16;
            self.score = *message.get("score")?.to_i32();
            self.avg_hit_time = *message.get("avg_hit_time")?.to_f32();
            self.accuracy = *message.get("accuracy")?.to_f32();
        }
        Some(())
    }

    pub fn write_message(&self, message: &mut Message) {
        message.insert("hit", Value::I32(self.hit as i32));
        message.insert("miss", Value::I32(self.miss as i32));
        message.insert("score", Value::I32(self.score));
        message.insert("avg_hit_time", Value::F32(self.avg_hit_time));
        message.insert("accuracy", Value::F32(self.accuracy));
    }
}

pub struct ClassicGameScene {
    ids: ClassicGameSceneIds,
    world: World,
    physics: GamePhysics,
    player_rigid_body_handle: RigidBodyHandle,
    game_timer: Timer,
    shoot_timer: Timer,
    game_start_timer: Timer,
    score: Score,
    game_running: bool,
    rng: SmallRng,
}

impl ClassicGameScene {
    pub fn new(_renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        let mut world = World::new();

        let mut physics = GamePhysics::new();

        // Ground
        let ground_rigid_body_handle = physics
            .rigid_body_set
            .insert(RigidBodyBuilder::new(RigidBodyType::Static).build());
        physics.collider_set.insert_with_parent(
            ColliderBuilder::new(SharedShape::cuboid(999.999, 0.01, 999.999)).build(),
            ground_rigid_body_handle,
            &mut physics.rigid_body_set,
        );

        // Player
        let player_rigid_body_handle = physics.rigid_body_set.insert(
            RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .translation(nalgebra::Vector3::new(0.0, 3.0, 0.0))
                .build(),
        );
        physics.collider_set.insert_with_parent(
            ColliderBuilder::new(SharedShape::cuboid(0.5, 0.5, 0.5)).build(),
            player_rigid_body_handle,
            &mut physics.rigid_body_set,
        );
        world.spawn((player_rigid_body_handle,));

        {
            let entity = world.reserve_entity();
            world.spawn_at(
                entity,
                (
                    Position(nalgebra::Vector3::new(0.0, 5.0, -10.0)),
                    physics.collider_set.insert(
                        ColliderBuilder::new(SharedShape::ball(0.5))
                            .user_data(entity.to_bits() as u128)
                            .build(),
                    ),
                    Target,
                ),
            );
        }

        Self {
            world,
            physics,
            player_rigid_body_handle,
            ids: ClassicGameSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
            score: Score::new(),
            shoot_timer: Timer::new_finished(),
            game_timer: Timer::new(Duration::new(100, 0)),
            game_start_timer: Timer::new(Duration::new(4, 0)),
            game_running: false,
            rng: SmallRng::from_entropy(),
        }
    }
}

impl Scene for ClassicGameScene {
    fn init(
        &mut self,
        _message: MaybeMessage,
        window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = true;

        {
            let entity = self.world.reserve_entity();
            let (objects, ref mut bound) = renderer.render_objects.next_static();
            objects.position = nalgebra::Vector3::new(0.0, 0.0, -20.0);
            objects.shape_type = ShapeType::Box;
            objects.shape_data1 = nalgebra::Vector4::new(20.0, 12.0, 0.1, 0.0);
            *bound = ObjectBound::Sphere(20.0);
            self.physics.collider_set.insert(
                ColliderBuilder::new(SharedShape::cuboid(
                    objects.shape_data1.x,
                    objects.shape_data1.y,
                    objects.shape_data1.z,
                ))
                .translation(objects.position)
                .user_data(entity.to_bits() as u128)
                .build(),
            );
            self.world.spawn_at(entity, (Label("Wall"),));
        }

        {
            let entity = self.world.reserve_entity();
            let (objects, ref mut bound) = renderer.render_objects.next_static();
            objects.position = nalgebra::Vector3::new(0.0, 0.0, 10.0);
            objects.shape_type = ShapeType::Box;
            objects.shape_data1 = nalgebra::Vector4::new(20.0, 5.0, 0.1, 0.0);
            *bound = ObjectBound::Sphere(20.0);
            self.physics.collider_set.insert(
                ColliderBuilder::new(SharedShape::cuboid(
                    objects.shape_data1.x,
                    objects.shape_data1.y,
                    objects.shape_data1.z,
                ))
                .translation(objects.position)
                .user_data(entity.to_bits() as u128)
                .build(),
            );
            self.world.spawn_at(entity, (Label("Wall"),));
        }

        {
            let entity = self.world.reserve_entity();
            let (objects, ref mut bound) = renderer.render_objects.next_static();
            objects.position = nalgebra::Vector3::new(-20.0, 0.0, -5.0);
            objects.shape_type = ShapeType::Box;
            objects.shape_data1 = nalgebra::Vector4::new(0.1, 5.0, 15.0, 0.0);
            *bound = ObjectBound::Sphere(15.0);
            self.physics.collider_set.insert(
                ColliderBuilder::new(SharedShape::cuboid(
                    objects.shape_data1.x,
                    objects.shape_data1.y,
                    objects.shape_data1.z,
                ))
                .translation(objects.position)
                .user_data(entity.to_bits() as u128)
                .build(),
            );
            self.world.spawn_at(entity, (Label("Wall"),));
        }

        {
            let entity = self.world.reserve_entity();
            let (objects, ref mut bound) = renderer.render_objects.next_static();
            objects.position = nalgebra::Vector3::new(20.0, 0.0, -5.0);
            objects.shape_type = ShapeType::Box;
            objects.shape_data1 = nalgebra::Vector4::new(0.1, 5.0, 15.0, 0.0);
            *bound = ObjectBound::Sphere(15.0);
            self.physics.collider_set.insert(
                ColliderBuilder::new(SharedShape::cuboid(
                    objects.shape_data1.x,
                    objects.shape_data1.y,
                    objects.shape_data1.z,
                ))
                .translation(objects.position)
                .user_data(entity.to_bits() as u128)
                .build(),
            );
            self.world.spawn_at(entity, (Label("Wall"),));
        }

        {
            let player_rigid_body = self
                .physics
                .rigid_body_set
                .get_mut(self.player_rigid_body_handle)
                .unwrap();
            renderer.camera.position = *player_rigid_body.translation();
        }

        window.set_is_cursor_grabbed(true);

        audio_context.global_sinks_map.remove("bgm");

        self.game_start_timer.reset(Duration::new(4, 0));
        self.game_start_timer.start();
        self.game_running = false;
    }

    fn update(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        _control_flow: &mut ControlFlow,
    ) -> SceneOp {
        renderer.camera.move_direction(input_manager.mouse_movement);

        let timer_duration = self.game_timer.get_duration();
        let sec = timer_duration.as_secs_f32();

        let ropa_font_id = *conrod_handle.get_font_id_map().get("ropa").unwrap();
        let mut ui_cell = conrod_handle.get_ui_mut().set_widgets();
        {
            conrod_core::widget::Canvas::new()
                .color(conrod_core::Color::Rgba(0.0, 0.0, 0.0, 0.0))
                .set(self.ids.canvas, &mut ui_cell);

            conrod_core::widget::Canvas::new()
                .color(conrod_core::Color::Rgba(1.0, 1.0, 1.0, 0.3))
                .mid_top_of(self.ids.canvas)
                .wh(conrod_core::Dimensions::new(100.0, 30.0))
                .set(self.ids.canvas_duration, &mut ui_cell);

            conrod_core::widget::Text::new(&format!(
                "{:02}:{:02}",
                (sec / 60.0) as i32,
                (sec % 60.0) as i32
            ))
            .font_id(ropa_font_id)
            .rgba(1.0, 1.0, 1.0, 1.0)
            .align_middle_x_of(self.ids.canvas_duration)
            .align_middle_y_of(self.ids.canvas_duration)
            .set(self.ids.duration_label, &mut ui_cell);
        }

        let mut scene_op = SceneOp::None;

        if !self.game_running {
            if self.game_start_timer.is_finished() {
                self.game_timer.start();
                self.game_running = true;
            } else {
                self.game_start_timer.update();
                conrod_core::widget::Text::new(&format!(
                    "{}",
                    self.game_start_timer.get_duration().as_secs()
                ))
                .font_id(ropa_font_id)
                .align_middle_x_of(self.ids.canvas)
                .align_middle_y_of(self.ids.canvas)
                .set(self.ids.start_duration_label, &mut ui_cell);
            }
        } else {
            self.game_timer.update();
            self.shoot_timer.update();

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

            let player_rigid_body = self
                .physics
                .rigid_body_set
                .get_mut(self.player_rigid_body_handle)
                .unwrap();
            renderer.camera.position = *player_rigid_body.translation();

            if input_manager.is_keyboard_press(&VirtualKeyCode::A) {
                renderer.camera.position -=
                    100.0 * delta_time * *renderer.camera.get_direction_right();
            } else if input_manager.is_keyboard_press(&VirtualKeyCode::D) {
                renderer.camera.position +=
                    100.0 * delta_time * *renderer.camera.get_direction_right();
            }

            if input_manager.is_keyboard_press(&VirtualKeyCode::W) {
                renderer.camera.position +=
                    100.0 * delta_time * *renderer.camera.get_direction_without_pitch();
            } else if input_manager.is_keyboard_press(&VirtualKeyCode::S) {
                renderer.camera.position -=
                    100.0 * delta_time * *renderer.camera.get_direction_without_pitch();
            }

            player_rigid_body.set_translation(renderer.camera.position, false);

            self.physics.query_pipeline.update(
                &self.physics.island_manager,
                &self.physics.rigid_body_set,
                &self.physics.collider_set,
            );

            if input_manager.is_mouse_press(&MouseButton::Left) && self.shoot_timer.is_finished() {
                self.shoot_timer.reset(Duration::new(0, 400000000));
                let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
                sink.append(
                    rodio::Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE_SHOOT.to_vec())))
                        .unwrap(),
                );
                audio_context.global_sinks_array.push(sink);

                let ray = Ray::new(
                    nalgebra::Point::from(
                        renderer.camera.position
                            + renderer.camera.get_direction().into_inner() * 1.0,
                    ),
                    renderer.camera.get_direction().into_inner(),
                );
                const MAX_RAYCAST_DISTANCE: f32 = 1000.0;
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
                    if let Ok(_) = self.world.get::<Target>(entity) {
                        // let sink = rodio::SpatialSink::try_new(
                        //     &audio_context.output_stream_handle,
                        //     [],
                        //     [],
                        //     [],
                        // )
                        // .unwrap();
                        // sink.append(
                        //     rodio::Decoder::new(BufReader::new(Cursor::new(
                        //         AUDIO_FILE_SHOOT.to_vec(),
                        //     )))
                        //     .unwrap(),
                        // );
                        // audio_context.global_sinks_array.push(sink);

                        let position = &mut self.world.get_mut::<Position>(entity).unwrap().0;
                        let x = self.rng.sample(Uniform::new(1.0, 3.0)) * 3.0;
                        let y = self.rng.sample(Uniform::new(1.0, 3.0)) * 3.0;
                        position.x += x;
                        position.y += y;
                        println!("{} {}", x, y);

                        self.score.hit += 1;
                    } else {
                        self.score.miss += 1;
                    }
                } else {
                    self.score.miss += 1;
                }
            }
        }

        drop(ui_cell);

        if sec <= 0.0 {
            scene_op = SceneOp::Push(
                Box::new(ClassicScoreScene::new(renderer, conrod_handle)),
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
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
        for (_id, (position, collider_handle, _)) in self
            .world
            .query_mut::<(&Position, &ColliderHandle, &Target)>()
        {
            let collider = self.physics.collider_set.get_mut(*collider_handle).unwrap();
            collider.set_translation(position.0);
            let (objects, ref mut bound) = renderer.render_objects.next();
            objects.position = position.0;
            objects.shape_type = ShapeType::Cylinder;
            let cam_to_obj = nalgebra::Unit::new_normalize(position.0 - renderer.camera.position);
            let inner_cam_to_obj = cam_to_obj.into_inner() * -0.1;

            objects.shape_data1.x = inner_cam_to_obj.x;
            objects.shape_data1.y = inner_cam_to_obj.y;
            objects.shape_data1.z = inner_cam_to_obj.z;
            objects.shape_data1.w = collider.shape().as_ball().unwrap().radius;

            objects.shape_data2 = objects.shape_data1 * -1.0;

            *bound = ObjectBound::Sphere(0.5);
        }
    }

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
    ) {
        renderer.render_objects.clear();
        window.set_is_cursor_grabbed(false);
        println!("Deinit game");
    }
}
