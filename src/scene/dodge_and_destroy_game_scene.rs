use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{Colorable, Positionable, Sizeable, Widget};
use std::collections::HashMap;

use std::io::{BufReader, Cursor};

use hecs::{Entity, World};
use instant::Instant;
use rapier3d::prelude::*;
use winit::event::{MouseButton, VirtualKeyCode};
use winit::event_loop::ControlFlow;

use crate::animation::InOutAnimation;
use crate::audio::Sink;
use crate::audio::{AudioContext, AUDIO_FILE_SHOOT};
use crate::database::Database;
use crate::enemy::gunman::Gunman;
use crate::enemy::swordman::Swordman;
use crate::enemy::HasMaterial;
use crate::entity::target::Target;
use crate::entity::Crate;
use crate::frustum::ObjectBound;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::physics::GamePhysics;
use crate::renderer::render_objects::MaterialType;
use crate::renderer::render_objects::ShapeType;
use crate::renderer::Renderer;
use crate::scene::classic_score_scene::ClassicScoreScene;
use crate::scene::pause_scene::PauseScene;
use crate::scene::{MaybeMessage, Message, Scene, SceneOp, Value};
use crate::systems::setup_player_collider::setup_player_collider;
use crate::systems::spawn_target::spawn_target;
use crate::systems::update_player_movement::update_player_position;
use crate::timer::{Stopwatch, Timer};
use crate::util::lerp;
use crate::window::Window;
use conrod_core::widget_ids;
use nalgebra::{Point3, Vector3};
use rand::distributions::Uniform;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Debug)]
enum Label {
    Wall,
    Gunman,
    Target,
    Swordman,
    Crate,
}

widget_ids! {
    pub struct DodgeAndDestroyGameSceneIds {
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
        message.insert("hit", Value::I32(self.hit as i32));
        message.insert("miss", Value::I32(self.miss as i32));
        message.insert("score", Value::I32(self.score));
        message.insert(
            "avg_hit_time",
            Value::F32(self.total_shoot_time / self.hit.max(1) as f32),
        );
    }
}

pub struct DodgeAndDestroyGameScene {
    ids: DodgeAndDestroyGameSceneIds,
    world: World,
    physics: GamePhysics,
    player_rigid_body_handle: RigidBodyHandle,
    game_timer: Timer,
    delta_shoot_time: Stopwatch,
    shoot_timer: Timer,
    game_start_timer: Timer,
    score: Score,
    game_running: bool,
    rng: SmallRng,
    shoot_animation: InOutAnimation,
    entity_to_remove: Vec<Entity>,
}

impl DodgeAndDestroyGameScene {
    pub fn new(_renderer: &mut Renderer, conrod_handle: &mut ConrodHandle) -> Self {
        let mut world = World::new();
        let mut physics = GamePhysics::new();

        // Ground
        physics.collider_set.insert(
            ColliderBuilder::new(SharedShape::cuboid(999.999, 0.1, 999.999))
                .translation(Vector3::new(0.0, 0.05, 0.0))
                .build(),
        );

        let player_rigid_body_handle = setup_player_collider(&mut physics);

        spawn_target(
            &mut world,
            &mut physics,
            Vector3::new(0.0, 5.0, -10.0),
            Target::new(),
        );

        {
            let entity = world.reserve_entity();
            let rigid_body_handle = physics.rigid_body_set.insert(
                RigidBodyBuilder::new(RigidBodyType::Dynamic)
                    .translation(Vector3::<f32>::new(2.0, 2.0, -2.0))
                    .lock_rotations()
                    .build(),
            );
            physics.collider_set.insert_with_parent(
                ColliderBuilder::new(SharedShape::capsule(
                    Point3::<f32>::new(0.0, 1.0, 0.0),
                    Point3::<f32>::new(0.0, -0.5, 0.0),
                    0.5,
                ))
                .user_data(entity.to_bits() as u128)
                .build(),
                rigid_body_handle,
                &mut physics.rigid_body_set,
            );
            world.spawn_at(entity, (Gunman::new(), rigid_body_handle, Label::Gunman));
        }

        {
            let entity = world.reserve_entity();
            let rigid_body_handle = physics.rigid_body_set.insert(
                RigidBodyBuilder::new(RigidBodyType::Dynamic)
                    .translation(Vector3::<f32>::new(-2.0, 2.0, -2.0))
                    .lock_rotations()
                    .build(),
            );
            physics.collider_set.insert_with_parent(
                ColliderBuilder::new(SharedShape::capsule(
                    Point3::<f32>::new(0.0, 1.0, 0.0),
                    Point3::<f32>::new(0.0, -0.5, 0.0),
                    0.5,
                ))
                .user_data(entity.to_bits() as u128)
                .build(),
                rigid_body_handle,
                &mut physics.rigid_body_set,
            );
            world.spawn_at(
                entity,
                (Swordman::new(), rigid_body_handle, Label::Swordman),
            );
        }

        Self {
            world,
            physics,
            player_rigid_body_handle,
            ids: DodgeAndDestroyGameSceneIds::new(conrod_handle.get_ui_mut().widget_id_generator()),
            score: Score::new(),
            delta_shoot_time: Stopwatch::new(),
            game_timer: Timer::new(100.0),
            game_start_timer: Timer::new_finished(),
            shoot_timer: Timer::new_finished(),
            game_running: false,
            rng: SmallRng::from_entropy(),
            shoot_animation: InOutAnimation::new(3.0, 5.0),
            entity_to_remove: Vec::new(),
        }
    }
}

impl Scene for DodgeAndDestroyGameScene {
    fn init(
        &mut self,
        _message: MaybeMessage,
        window: &mut Window,
        renderer: &mut Renderer,
        _conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        _database: &mut Database,
    ) {
        renderer.is_render_gui = true;
        renderer.is_render_game = true;
        //
        // {
        //     let entity = self.world.reserve_entity();
        //     let (objects, ref mut bound) = renderer.render_objects.next_static();
        //     objects.position = nalgebra::Vector3::new(0.0, 0.0, -20.0);
        //     objects.shape_type_material_ids.0 = ShapeType::Box;
        //     objects.shape_type_material_ids.1 = MaterialType::Checker;
        //     objects.shape_data1 = nalgebra::Vector4::new(20.0, 12.0, 1.0, 0.0);
        //     *bound = ObjectBound::Sphere(20.0);
        //     self.physics.collider_set.insert(
        //         ColliderBuilder::new(SharedShape::cuboid(
        //             objects.shape_data1.x,
        //             objects.shape_data1.y,
        //             objects.shape_data1.z,
        //         ))
        //         .translation(objects.position)
        //         .user_data(entity.to_bits() as u128)
        //         .build(),
        //     );
        //     self.world.spawn_at(entity, (Label::Wall,));
        // }
        //
        // {
        //     let entity = self.world.reserve_entity();
        //     let (objects, ref mut bound) = renderer.render_objects.next_static();
        //     objects.position = nalgebra::Vector3::new(0.0, 0.0, 10.0);
        //     objects.shape_type_material_ids.0 = ShapeType::Box;
        //     objects.shape_type_material_ids.1 = MaterialType::Checker;
        //     objects.shape_data1 = nalgebra::Vector4::new(20.0, 5.0, 1.0, 0.0);
        //     *bound = ObjectBound::Sphere(20.0);
        //     self.physics.collider_set.insert(
        //         ColliderBuilder::new(SharedShape::cuboid(
        //             objects.shape_data1.x,
        //             objects.shape_data1.y,
        //             objects.shape_data1.z,
        //         ))
        //         .translation(objects.position)
        //         .user_data(entity.to_bits() as u128)
        //         .build(),
        //     );
        //     self.world.spawn_at(entity, (Label::Wall,));
        // }
        //
        // {
        //     let entity = self.world.reserve_entity();
        //     let (objects, ref mut bound) = renderer.render_objects.next_static();
        //     objects.position = nalgebra::Vector3::new(-20.0, 0.0, -5.0);
        //     objects.shape_type_material_ids.0 = ShapeType::Box;
        //     objects.shape_type_material_ids.1 = MaterialType::Checker;
        //     objects.shape_data1 = nalgebra::Vector4::new(1.0, 5.0, 15.0, 0.0);
        //     *bound = ObjectBound::Sphere(15.0);
        //     self.physics.collider_set.insert(
        //         ColliderBuilder::new(SharedShape::cuboid(
        //             objects.shape_data1.x,
        //             objects.shape_data1.y,
        //             objects.shape_data1.z,
        //         ))
        //         .translation(objects.position)
        //         .user_data(entity.to_bits() as u128)
        //         .build(),
        //     );
        //     self.world.spawn_at(entity, (Label::Wall,));
        // }
        //
        // {
        //     let entity = self.world.reserve_entity();
        //     let (objects, ref mut bound) = renderer.render_objects.next_static();
        //     objects.position = nalgebra::Vector3::new(20.0, 0.0, -5.0);
        //     objects.shape_type_material_ids.0 = ShapeType::Box;
        //     objects.shape_type_material_ids.1 = MaterialType::Checker;
        //     objects.shape_data1 = nalgebra::Vector4::new(1.0, 5.0, 15.0, 0.0);
        //     *bound = ObjectBound::Sphere(15.0);
        //     self.physics.collider_set.insert(
        //         ColliderBuilder::new(SharedShape::cuboid(
        //             objects.shape_data1.x,
        //             objects.shape_data1.y,
        //             objects.shape_data1.z,
        //         ))
        //         .translation(objects.position)
        //         .user_data(entity.to_bits() as u128)
        //         .build(),
        //     );
        //     self.world.spawn_at(entity, (Label::Wall,));
        // }

        {
            let player_rigid_body = self
                .physics
                .rigid_body_set
                .get_mut(self.player_rigid_body_handle)
                .unwrap();
            renderer.camera.position = *player_rigid_body.translation();
        }

        {
            let entity = self.world.reserve_entity();
            let rb_handle = self.physics.rigid_body_set.insert(
                RigidBodyBuilder::new(RigidBodyType::Dynamic)
                    .user_data(entity.to_bits() as u128)
                    .build(),
            );
            self.physics.collider_set.insert_with_parent(
                ColliderBuilder::new(SharedShape::cuboid(1.0, 1.0, 1.0))
                    .user_data(entity.to_bits() as u128)
                    .build(),
                rb_handle,
                &mut self.physics.rigid_body_set,
            );
            self.world
                .spawn_at(entity, (Label::Crate, Crate, rb_handle));
        }

        window.set_is_cursor_grabbed(true);

        audio_context.global_sinks_map.remove("bgm");

        self.game_running = false;
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
        renderer.camera.move_direction(input_manager.mouse_movement);

        let sec = self.game_timer.get_duration();

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
            .rgba(1.0, 1.0, 1.0, 1.0)
            .middle_of(self.ids.canvas_duration)
            .set(self.ids.duration_label, &mut ui_cell);
        }

        let mut scene_op = SceneOp::None;

        if !self.game_running {
            if self.game_start_timer.is_finished() {
                self.game_running = true;
            } else {
                self.game_start_timer.update(delta_time);
                conrod_core::widget::Text::new(&format!(
                    "{:.1}",
                    self.game_start_timer.get_duration()
                ))
                .align_middle_x_of(self.ids.canvas)
                .align_middle_y_of(self.ids.canvas)
                .set(self.ids.start_duration_label, &mut ui_cell);
            }
        } else {
            self.game_timer.update(delta_time);
            self.shoot_timer.update(delta_time);
            self.delta_shoot_time.update(delta_time);

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
                }
            }
            for entity in self.entity_to_remove.iter() {
                self.world.despawn(*entity).unwrap();
            }
            self.entity_to_remove.clear();

            self.shoot_animation.update(delta_time);
            renderer.rendering_info.fov_shootanim.y = lerp(
                0.0f32,
                -20.0f32.to_radians(),
                self.shoot_animation.get_value(),
            );
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

            let player_position = update_player_position(
                delta_time,
                input_manager,
                &mut renderer.camera,
                &mut self.physics,
                self.player_rigid_body_handle,
            );

            for (_id, (gunman, rb_handle)) in
                self.world.query_mut::<(&mut Gunman, &RigidBodyHandle)>()
            {
                let gunman_rigid_body = self.physics.rigid_body_set.get_mut(*rb_handle).unwrap();
                let mut gunman_pos = *gunman_rigid_body.translation();
                gunman.update(delta_time, &mut gunman_pos, &player_position);
                gunman_rigid_body.set_translation(gunman_pos, true);
            }
            for (_id, (swordman, rb_handle)) in
                self.world.query_mut::<(&mut Swordman, &RigidBodyHandle)>()
            {
                let swordman_rigid_body = self.physics.rigid_body_set.get_mut(*rb_handle).unwrap();
                let mut swordman_pos = *swordman_rigid_body.translation();
                swordman.update(delta_time, &mut swordman_pos, &player_position);
                swordman_rigid_body.set_translation(swordman_pos, true);
            }

            self.physics.query_pipeline.update(
                &self.physics.island_manager,
                &self.physics.rigid_body_set,
                &self.physics.collider_set,
            );

            if input_manager.is_mouse_press(&MouseButton::Left) && self.shoot_timer.is_finished() {
                self.shoot_animation.trigger();
                self.shoot_timer.reset(0.4);
                let sink = rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
                sink.append(
                    rodio::Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE_SHOOT.to_vec())))
                        .unwrap(),
                );
                audio_context.global_sinks_array.push(Sink::Regular(sink));

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
                    // if let Ok(target_pos) = self.world.get::<Position>(entity) {
                    //     let sink =
                    //         rodio::Sink::try_new(&audio_context.output_stream_handle).unwrap();
                    //     sink.append(
                    //         rodio::Decoder::new(BufReader::new(Cursor::new(
                    //             AUDIO_FILE_SHOOTED.to_vec(),
                    //         )))
                    //         .unwrap(),
                    //     );
                    //     audio_context.global_sinks_array.push(Sink::Regular(sink));
                    // }
                    let mut need_to_spawn = false;

                    if let Ok(label) = self.world.get::<Label>(entity) {
                        match *label {
                            Label::Target => {
                                if let Ok(mut target) = self.world.get_mut::<Target>(entity) {
                                    if !target.is_shooted() {
                                        let _time_now = Instant::now();
                                        let shoot_time = self.delta_shoot_time.get_duration();
                                        self.delta_shoot_time.reset();
                                        self.score.total_shoot_time += shoot_time;
                                        need_to_spawn = true;
                                        target.shooted();
                                        self.score.score +=
                                            ((300.0 * (3.0 - shoot_time)) as i32).max(0);
                                        self.score.hit += 1;
                                    }
                                }
                            }
                            Label::Gunman => {
                                let mut gunman = self.world.get_mut::<Gunman>(entity).unwrap();
                                gunman.hit();
                            }
                            Label::Swordman => {
                                let mut swordman = self.world.get_mut::<Swordman>(entity).unwrap();
                                swordman.hit();
                            }
                            _ => {}
                        };
                    }
                    if need_to_spawn {
                        let pos = nalgebra::Vector3::new(
                            self.rng.sample(Uniform::new(-5.0, 5.0)),
                            self.rng.sample(Uniform::new(0.5, 5.0)),
                            -10.0,
                        );
                        let new_entity = self.world.reserve_entity();
                        self.world.spawn_at(
                            new_entity,
                            (
                                self.physics.collider_set.insert(
                                    ColliderBuilder::new(SharedShape::ball(0.5))
                                        .user_data(new_entity.to_bits() as u128)
                                        .translation(pos)
                                        .build(),
                                ),
                                Target::new(),
                            ),
                        );
                    }
                } else {
                    self.score.score -= 100;
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
        _input_manager: &InputManager,
        _delta_time: f32,
        _conrod_handle: &mut ConrodHandle,
        _audio_context: &mut AudioContext,
    ) {
        for (_id, (gunman, rb_handle)) in self.world.query_mut::<(&Gunman, &RigidBodyHandle)>() {
            let rb = self.physics.rigid_body_set.get(*rb_handle).unwrap();

            let (objects, ref mut bound) = renderer.render_objects.next();
            objects.position = *rb.translation();
            objects.shape_data2.y = gunman.get_rotation();
            objects.shape_type_material_ids.0 = ShapeType::Gunman;
            objects.shape_type_material_ids.1 = gunman.get_material();
            objects.shape_type_material_ids.2 = MaterialType::Black;
            objects.rotation = rb.rotation().to_homogeneous();

            *bound = ObjectBound::None;
        }

        for (_id, (swordman, rb_handle)) in self.world.query_mut::<(&Swordman, &RigidBodyHandle)>()
        {
            let rb = self.physics.rigid_body_set.get(*rb_handle).unwrap();

            let (objects, ref mut bound) = renderer.render_objects.next();
            objects.position = *rb.translation();
            objects.shape_data2.y = swordman.get_rotation();
            objects.shape_type_material_ids.0 = ShapeType::Swordman;
            objects.shape_type_material_ids.1 = swordman.get_material();
            objects.shape_type_material_ids.2 = MaterialType::Green;
            objects.rotation = rb.rotation().to_homogeneous();

            *bound = ObjectBound::None;
        }

        for (_id, (_crate, rb_handle)) in self.world.query_mut::<(&Crate, &RigidBodyHandle)>() {
            let rb = self.physics.rigid_body_set.get(*rb_handle).unwrap();
            let collider = self.physics.collider_set.get(rb.colliders()[0]).unwrap();

            let (objects, ref mut bound) = renderer.render_objects.next();
            objects.position = *rb.translation();
            objects.shape_type_material_ids.0 = ShapeType::Box;
            objects.shape_type_material_ids.1 = MaterialType::Crate;
            objects.rotation = rb.rotation().to_homogeneous();

            let shape = collider.shape().as_cuboid().unwrap();
            objects.shape_data1.x = shape.half_extents.x;
            objects.shape_data1.y = shape.half_extents.y;
            objects.shape_data1.z = shape.half_extents.z;

            *bound = ObjectBound::Sphere(
                (objects.shape_data1.x.powf(2.0)
                    * objects.shape_data1.y.powf(2.0)
                    * objects.shape_data1.z.powf(2.0))
                .sqrt(),
            );
        }

        for (_id, (collider_handle, target)) in self.world.query_mut::<(&ColliderHandle, &Target)>()
        {
            let collider = self.physics.collider_set.get(*collider_handle).unwrap();
            let (objects, ref mut bound) = renderer.render_objects.next();
            objects.position = *collider.translation();
            objects.shape_type_material_ids.0 = ShapeType::Sphere;
            objects.shape_type_material_ids.1 = target.get_material();
            // let cam_to_obj = nalgebra::Unit::new_normalize(position.0 - renderer.camera.position);
            // let inner_cam_to_obj = cam_to_obj.into_inner() * -0.1;

            // objects.shape_data1.x = inner_cam_to_obj.x;
            // objects.shape_data1.y = inner_cam_to_obj.y;
            // objects.shape_data1.z = inner_cam_to_obj.z;
            objects.shape_data1.x = collider.shape().as_ball().unwrap().radius;

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
        _database: &mut Database,
    ) {
        renderer.rendering_info.fov_shootanim.y = 0.0;
        renderer.render_objects.clear();
        window.set_is_cursor_grabbed(false);
    }
}
