use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::{widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget};
use hecs::World;
use rapier3d::prelude::*;
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;

use pause_scene::PauseScene;
use settings_scene::SettingsScene;

use crate::audio::{AudioContext, SINK_ID_MAIN_MENU_BGM};
use crate::gui::ConrodHandle;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::window::Window;

pub mod classic_game_scene;
pub mod main_menu_scene;
pub mod pause_scene;
pub mod settings_scene;

pub enum SceneOp {
    None,
    Push(Box<dyn Scene>),
    Pop(u8),
    Replace(Box<dyn Scene>),
}

pub trait Scene {
    fn init(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    );

    fn update(
        &mut self,
        renderer: &mut Renderer,
        input_manager: &InputManager,
        delta_time: f32,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
        control_flow: &mut ControlFlow,
    ) -> SceneOp;

    fn deinit(
        &mut self,
        window: &mut Window,
        renderer: &mut Renderer,
        conrod_handle: &mut ConrodHandle,
        audio_context: &mut AudioContext,
    );
}

pub const MARGIN: conrod_core::Scalar = 30.0;

pub struct GamePhysics {
    pub gravity: nalgebra::Vector3<f32>,
    pub rigid_body_set: rapier3d::prelude::RigidBodySet,
    pub collider_set: rapier3d::prelude::ColliderSet,
    pub integration_parameters: rapier3d::prelude::IntegrationParameters,
    pub physics_pipeline: rapier3d::prelude::PhysicsPipeline,
    pub island_manager: rapier3d::prelude::IslandManager,
    pub broad_phase: rapier3d::prelude::BroadPhase,
    pub narrow_phase: rapier3d::prelude::NarrowPhase,
    pub joint_set: rapier3d::prelude::JointSet,
    pub ccd_solver: rapier3d::prelude::CCDSolver,
}

impl GamePhysics {
    fn new() -> Self {
        Self {
            gravity: nalgebra::Vector3::new(0.0, -9.81, 0.0),
            rigid_body_set: rapier3d::prelude::RigidBodySet::new(),
            collider_set: rapier3d::prelude::ColliderSet::new(),
            integration_parameters: rapier3d::prelude::IntegrationParameters::default(),
            physics_pipeline: rapier3d::prelude::PhysicsPipeline::new(),
            island_manager: rapier3d::prelude::IslandManager::new(),
            broad_phase: rapier3d::prelude::BroadPhase::new(),
            narrow_phase: rapier3d::prelude::NarrowPhase::new(),
            joint_set: rapier3d::prelude::JointSet::new(),
            ccd_solver: rapier3d::prelude::CCDSolver::new(),
        }
    }
}
