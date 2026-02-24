//! An encapsulated container for entities, systems, and resources.

use crate::ecs::{resources::Resources, system::SystemScheduler, world::World};

/// A single loaded scene (level, menu, UI overlay) running in the engine.
pub struct Scene {
    /// The ECS world for this scene.
    pub world: World,
    /// The scheduler for systems running within this scene.
    pub scheduler: SystemScheduler,
    /// Scene-local resources.
    pub resources: Resources,
    /// Indicates if this scene should update/render when not at the top of the stack.
    pub is_modal: bool,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    /// Create a new, blank scene.
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: SystemScheduler::new(),
            resources: Resources::new(),
            is_modal: false,
        }
    }

    /// Mark this scene as modal (it blocks execution of scenes underneath it).
    pub fn set_modal(&mut self, modal: bool) {
        self.is_modal = modal;
    }

    /// Run a specific stage of the scheduler onto the world.
    pub fn run_stage(&mut self, stage: crate::ecs::system::SystemStage) {
        self.scheduler.run_stage(stage, &mut self.world, &mut self.resources);
    }
}
