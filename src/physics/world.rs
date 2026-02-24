//! The Rapier2D physics world wrapper.

use glam::Vec2;
use rapier2d::prelude::*;

/// The global physics world simulating rigid bodies and colliders.
pub struct PhysicsWorld {
    /// Rigid body storage.
    pub bodies: RigidBodySet,
    /// Collider storage.
    pub colliders: ColliderSet,
    /// Simulates gravity and other parameters.
    pub integration_parameters: IntegrationParameters,
    /// Physics pipeline performing the actual timestep.
    pub physics_pipeline: PhysicsPipeline,
    /// Handles islands (sleeping bodies).
    pub island_manager: IslandManager,
    /// Broad-phase collision detection.
    pub broad_phase: BroadPhase,
    /// Narrow-phase collision detection.
    pub narrow_phase: NarrowPhase,
    /// Joint constraints.
    pub impulse_joints: ImpulseJointSet,
    /// Multi-body joints.
    pub multibody_joints: MultibodyJointSet,
    /// Detects continuous collision.
    pub ccd_solver: CCDSolver,
    /// Query pipeline for raycasts.
    pub query_pipeline: QueryPipeline,
    /// Gravity vector.
    pub gravity: Vector<Real>,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new(Vec2::new(0.0, -9.81))
    }
}

impl PhysicsWorld {
    /// Create a new physics world with the specified gravity.
    pub fn new(gravity: Vec2) -> Self {
        Self {
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            gravity: vector![gravity.x, gravity.y],
        }
    }

    /// Step the physics simulation forward by `dt` seconds.
    pub fn step(&mut self, dt: f32) {
        self.integration_parameters.dt = dt;
        
        // This closure is an event handler for collisions, but Veltrix uses ECS polling usually.
        let physics_hooks = ();
        let event_handler = ();
        
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &physics_hooks,
            &event_handler,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_steps_without_panic() {
        let mut pw = PhysicsWorld::new(Vec2::new(0.0, -9.81));
        pw.step(1.0 / 60.0);
    }
}
