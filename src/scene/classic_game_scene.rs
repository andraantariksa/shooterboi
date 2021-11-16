use hecs::World;
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodyHandle, RigidBodyType};
use rapier3d::geometry::{ColliderBuilder, SharedShape};
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;

use crate::audio::{AudioContext, SINK_ID_MAIN_MENU_BGM};
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::{Renderer, ShapeType};
use crate::scene::pause_scene::PauseScene;
use crate::scene::{GamePhysics, Scene, SceneOp};
use crate::window::Window;

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

        renderer.render_objects[0].position = nalgebra::Vector3::new(0.0, 0.0, -20.0);
        renderer.render_objects[0].shape_type = ShapeType::Box;
        renderer.render_objects[0].shape_data1 = nalgebra::Vector4::new(20.0, 12.0, 0.1, 0.0);

        renderer.render_objects[1].position = nalgebra::Vector3::new(0.0, 0.0, 10.0);
        renderer.render_objects[1].shape_type = ShapeType::Box;
        renderer.render_objects[1].shape_data1 = nalgebra::Vector4::new(20.0, 5.0, 0.1, 0.0);

        renderer.render_objects[2].position = nalgebra::Vector3::new(-20.0, 0.0, -5.0);
        renderer.render_objects[2].shape_type = ShapeType::Box;
        renderer.render_objects[2].shape_data1 = nalgebra::Vector4::new(0.1, 5.0, 15.0, 0.0);

        renderer.render_objects[3].position = nalgebra::Vector3::new(20.0, 0.0, -5.0);
        renderer.render_objects[3].shape_type = ShapeType::Box;
        renderer.render_objects[3].shape_data1 = nalgebra::Vector4::new(0.1, 5.0, 15.0, 0.0);

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

        scene_op
    }

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    ) {
        window.set_is_cursor_grabbed(false);
    }
}
