//! Deferred world mutation commands.
//!
//! Prevents runtime borrow panics by queuing spawns, despawns, and component
//! insertions to be applied linearly after all systems run.

use std::any::Any;
use std::collections::VecDeque;

use super::world::{Entity, World};

/// A command that modifies the World.
type CommandFn = Box<dyn FnOnce(&mut World) + Send + Sync>;

/// Collects deferred operations on the `World`.
#[derive(Default)]
pub struct Commands {
    queue: VecDeque<CommandFn>,
}

impl Commands {
    /// Create an empty command queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawn a new entity.
    ///
    /// Since the real entity ID cannot be returned immediately (it hasn't been spawned in
    /// the World yet), this pushes a spawn command. For immediate Entity access, build
    /// an `EntityBuilder` pattern, but Veltrix uses direct callbacks here.
    pub fn spawn<F>(&mut self, f: F)
    where
        F: FnOnce(Entity, &mut World) + Send + Sync + 'static,
    {
        self.queue.push_back(Box::new(move |world| {
            let entity = world.spawn();
            f(entity, world);
        }));
    }

    /// Despawn an entity.
    pub fn despawn(&mut self, entity: Entity) {
        self.queue.push_back(Box::new(move |world| {
            world.despawn(entity);
        }));
    }

    /// Insert a component on an existing entity.
    pub fn insert<T: Send + Sync + 'static>(&mut self, entity: Entity, component: T) {
        self.queue.push_back(Box::new(move |world| {
            world.insert(entity, component);
        }));
    }

    /// Remove a component from an existing entity.
    pub fn remove<T: Any + Send + Sync + 'static>(&mut self, entity: Entity) {
        self.queue.push_back(Box::new(move |world| {
            world.remove::<T>(entity);
        }));
    }

    /// Apply all queued commands to the world, emptying the queue.
    pub fn apply(&mut self, world: &mut World) {
        // Drain into a local vector to allow commands to add new commands (re-entrancy safely avoided if simple loop).
        while let Some(command) = self.queue.pop_front() {
            command(world);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_apply_defer_spawn_insert() {
        let mut commands = Commands::new();
        let mut world = World::new();
        
        commands.spawn(|e, w| {
            w.insert(e, 100_u32);
        });
        
        // No entities initially
        assert_eq!(world.entities().count(), 0);
        
        commands.apply(&mut world);
        
        // After apply, Entity exists.
        assert_eq!(world.entities().count(), 1);
        let e = world.entities().next().unwrap();
        assert_eq!(world.get::<u32>(e), Some(&100_u32));
    }

    #[test]
    fn despawn_deferred() {
        let mut commands = Commands::new();
        let mut world = World::new();
        let e = world.spawn();
        
        commands.despawn(e);
        assert_eq!(world.entities().count(), 1);
        
        commands.apply(&mut world);
        assert_eq!(world.entities().count(), 0);
    }
}
