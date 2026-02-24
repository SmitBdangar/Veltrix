//! System trait, stages, and the SystemScheduler for the ECS.

use super::{resources::Resources, world::World};

/// A lifecycle stage for running ECS systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemStage {
    /// Runs before anything else each frame.
    PreUpdate,
    /// Runs on a fixed timestep (e.g. physics).
    FixedUpdate,
    /// Runs every frame with variable delta time.
    Update,
    /// Runs after Update.
    LateUpdate,
    /// Runs during rendering with interpolation alpha.
    Render,
}

/// A System is a boxed closure or struct that operates on the World and Resources.
pub trait System: Send + Sync {
    /// Execute the system logic.
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}

/// A simple System implementation wrapping a closure.
struct SystemFn {
    name: String,
    f: Box<dyn FnMut(&mut World, &mut Resources) + Send + Sync>,
}

impl System for SystemFn {
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        (self.f)(world, resources)
    }
}

/// Schedules and runs ECS systems in defined stages.
#[derive(Default)]
pub struct SystemScheduler {
    stages: std::collections::HashMap<SystemStage, Vec<Box<dyn System>>>,
}

impl SystemScheduler {
    /// Create an empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a system to a specific stage.
    pub fn add_system<F>(&mut self, stage: SystemStage, name: &str, system: F)
    where
        F: FnMut(&mut World, &mut Resources) + Send + Sync + 'static,
    {
        self.stages.entry(stage).or_default().push(Box::new(SystemFn {
            name: name.to_string(),
            f: Box::new(system),
        }));
    }

    /// Run all systems registered for a given stage sequentially.
    ///
    /// (A more complex ECS would build a dependency graph and run disjoint systems
    /// in parallel using `rayon`, but Veltrix uses sequential execution for simplicity).
    pub fn run_stage(&mut self, stage: SystemStage, world: &mut World, resources: &mut Resources) {
        if let Some(systems) = self.stages.get_mut(&stage) {
            for sys in systems.iter_mut() {
                // In a real profiler, we would wrap this call with a timer.
                sys.run(world, resources);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_runs_system() {
        let mut scheduler = SystemScheduler::new();
        let mut world = World::new();
        let mut res = Resources::new();
        
        res.insert(0_u32);

        scheduler.add_system(SystemStage::Update, "AddOne", |_, r| {
            *r.get_mut::<u32>().unwrap() += 1;
        });

        assert_eq!(*res.get::<u32>().unwrap(), 0);
        scheduler.run_stage(SystemStage::Update, &mut world, &mut res);
        assert_eq!(*res.get::<u32>().unwrap(), 1);
        
        // Ensure other stages don't run it
        scheduler.run_stage(SystemStage::FixedUpdate, &mut world, &mut res);
        assert_eq!(*res.get::<u32>().unwrap(), 1);
    }
}
