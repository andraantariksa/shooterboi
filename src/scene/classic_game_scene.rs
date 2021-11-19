use std::collections::HashMap;
use std::io::Cursor;

use hecs::{Entity, World};
use rapier3d::prelude::*;
use winit::event::{MouseButton, VirtualKeyCode};
use winit::event_loop::ControlFlow;

use crate::audio::{AudioContext, SINK_ID_MAIN_MENU_BGM};
use crate::camera::ObjectBound;
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::physics::GamePhysics;
use crate::renderer::{Renderer, ShapeType};
use crate::scene::pause_scene::PauseScene;
use crate::scene::{Scene, SceneOp};
use crate::window::Window;

#[derive(Debug)]
struct Label(&'static str);

pub struct Target;

pub struct Position(nalgebra::Vector3<f32>);

pub struct ClassicGameScene {
    world: World,
    physics: GamePhysics,
    player_rigid_body_handle: RigidBodyHandle,
}

impl ClassicGameScene {
    pub fn new(renderer: &mut Renderer) -> Self {
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

        Self {
            world,
            physics,
            player_rigid_body_handle,
        }
    }
}

impl Scene for ClassicGameScene {
    fn init(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
        renderer.is_render_gui = false;
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
            let entity = self.world.reserve_entity();
            self.world.spawn_at(
                entity,
                (
                    Position(nalgebra::Vector3::new(0.0, 5.0, -10.0)),
                    self.physics.collider_set.insert(
                        ColliderBuilder::new(SharedShape::ball(0.5))
                            .user_data(entity.to_bits() as u128)
                            .build(),
                    ),
                    Target,
                ),
            );
        }

        renderer.rendering_info.queue_count = 10;

        window.set_is_cursor_grabbed(true);
        audio_context.get_sink_mut(SINK_ID_MAIN_MENU_BGM).stop();
    }

    fn update(
        &mut self,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        control_flow: &mut ControlFlow,
    ) -> SceneOp {
        let mut scene_op = SceneOp::None;

        if input_manager.is_keyboard_press(&VirtualKeyCode::Escape) {
            scene_op = SceneOp::Push(Box::new(PauseScene::new(renderer, conrod_handle)));
        }

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

        let mut player_rigid_body = self
            .physics
            .rigid_body_set
            .get_mut(self.player_rigid_body_handle)
            .unwrap();
        renderer.camera.position = *player_rigid_body.translation();

        if input_manager.is_keyboard_press(&VirtualKeyCode::A) {
            renderer.camera.position -= 100.0 * delta_time * *renderer.camera.get_direction_right();
        } else if input_manager.is_keyboard_press(&VirtualKeyCode::D) {
            renderer.camera.position += 100.0 * delta_time * *renderer.camera.get_direction_right();
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

        if input_manager.is_mouse_press(&MouseButton::Left) {
            let shoot_audio = audio_context
                .output_stream_handle
                .play_once(Cursor::new(
                    include_bytes!("../../assets/audio/shoot.wav").to_vec(),
                ))
                .unwrap();
            audio_context.global_sinks.push(shoot_audio);

            let ray = Ray::new(
                nalgebra::Point::from(
                    renderer.camera.position + renderer.camera.get_direction().into_inner() * 1.0,
                ),
                renderer.camera.get_direction().into_inner(),
            );
            const MAX_RAYCAST_DISTANCE: f32 = 1000.0;
            if let Some((handle, distance)) = self.physics.query_pipeline.cast_ray(
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
                    let shoot_audio = audio_context
                        .output_stream_handle
                        .play_once(Cursor::new(
                            include_bytes!("../../assets/audio/shooted.wav").to_vec(),
                        ))
                        .unwrap();
                    audio_context.global_sinks.push(shoot_audio);
                    let position = &mut self.world.get_mut::<Position>(entity).unwrap().0;
                    position.x += 0.5;
                }
            }
        }

        for (id, (position, collider_handle, _)) in self
            .world
            .query_mut::<(&Position, &ColliderHandle, &Target)>()
        {
            let collider = self.physics.collider_set.get_mut(*collider_handle).unwrap();
            collider.set_translation(position.0);
            let (objects, ref mut bound) = renderer.render_objects.next();
            objects.position = position.0;
            objects.shape_type = ShapeType::Sphere;
            objects.shape_data1.x = collider.shape().as_ball().unwrap().radius;
            *bound = ObjectBound::Sphere(0.5);
        }

        scene_op
    }

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
        renderer.render_objects.clear();
        window.set_is_cursor_grabbed(false);
    }
}
