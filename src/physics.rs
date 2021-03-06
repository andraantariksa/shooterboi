use crossbeam::channel::Receiver;
use rapier3d::prelude::*;

pub struct GamePhysics {
    pub gravity: nalgebra::Vector3<f32>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub joint_set: JointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub interaction_groups: InteractionGroups,
    pub event_handler: ChannelEventCollector,
    pub contact_recv: Receiver<ContactEvent>,
    pub intersection_recv: Receiver<IntersectionEvent>,
}

impl GamePhysics {
    pub(crate) fn new() -> Self {
        let (contact_send, contact_recv) = crossbeam::channel::unbounded();
        let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
        Self {
            gravity: nalgebra::Vector3::new(0.0, -9.81, 0.0),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            interaction_groups: InteractionGroups::all(),
            event_handler: ChannelEventCollector::new(intersection_send, contact_send),
            contact_recv,
            intersection_recv,
        }
    }
}
