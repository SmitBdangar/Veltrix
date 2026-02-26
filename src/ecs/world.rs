//! The core ECS World and Entity definitions, backed by `hecs` for archetype storage.

use std::any::Any;

pub use hecs::Entity;

/// Extension trait to provide a `null()` equivalent for `hecs::Entity`
pub trait EntityExt {
    fn null() -> Self;
}

impl EntityExt for Entity {
    fn null() -> Self {
        Entity::from_bits(0).unwrap() // Usually safe as a null sentinel since id 0 is never returned normally.
    }
}

/// The ECS World, holding all entities and their components.
/// Uses `hecs` under the hood for contiguous archetype storage.
#[derive(Default)]
pub struct World {
    inner: hecs::World,
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("entity_count", &self.inner.len())
            .finish()
    }
}

impl World {
    /// Create an empty World.
    pub fn new() -> Self {
        Self {
            inner: hecs::World::new(),
        }
    }

    /// Spawn a new entity with no components.
    pub fn spawn(&mut self) -> Entity {
        self.inner.spawn(())
    }

    /// Despawn an entity and remove all its components.
    pub fn despawn(&mut self, entity: Entity) -> bool {
        self.inner.despawn(entity).is_ok()
    }

    /// Add a component to an entity.
    pub fn insert<T: Send + Sync + 'static>(&mut self, entity: Entity, component: T) {
        let _ = self.inner.insert_one(entity, component);
    }

    /// Remove a component from an entity and return it.
    pub fn remove<T: Send + Sync + 'static>(&mut self, entity: Entity) -> Option<T> {
        self.inner.remove_one::<T>(entity).ok()
    }

    /// Get an immutable reference to a component for a specific entity.
    pub fn get<T: Send + Sync + 'static>(&self, entity: Entity) -> Option<hecs::Ref<'_, T>> {
        self.inner.get::<&T>(entity).ok()
    }

    /// Get a mutable reference to a component for a specific entity.
    pub fn get_mut<T: Send + Sync + 'static>(&self, entity: Entity) -> Option<hecs::RefMut<'_, T>> {
        self.inner.get::<&mut T>(entity).ok()
    }

    /// Iterator over all alive entities.
    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.inner.iter().map(|e_ref| e_ref.entity())
    }

    /// Exposes the inner `hecs::World` for advanced queries.
    pub fn inner(&self) -> &hecs::World {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut hecs::World {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_and_despawn() {
        let mut world = World::new();
        let e = world.spawn();
        assert!(world.entities().count() == 1);
        assert!(world.despawn(e));
        assert!(world.entities().count() == 0);
    }

    #[test]
    fn component_insert_and_get() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, 42_u32);
        assert_eq!(*world.get::<u32>(e).unwrap(), 42);
    }

    #[test]
    fn component_remove() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, 10_i32);
        let removed = world.remove::<i32>(e);
        assert_eq!(removed, Some(10));
        assert!(world.get::<i32>(e).is_none());
    }
}
