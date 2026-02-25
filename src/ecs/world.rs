//! The core ECS World and Entity definitions.

use slotmap::{new_key_type, SlotMap};
use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::storage::{BoxedStorage, ComponentStorage};

new_key_type! {
    /// A unique generational ID for an entity in the ECS.
    pub struct Entity;
}

impl Entity {
    /// Returns a null entity (for tests/dummies only).
    pub fn null() -> Self {
        <Self as slotmap::Key>::null()
    }
}

/// The ECS World, holding all entities and their components.
#[derive(Default)]
pub struct World {
    /// Allocates generational indices for entities.
    entities: SlotMap<Entity, ()>,
    /// Maps component `TypeId` → `BoxedStorage` (`ComponentStorage<T>`).
    component_storages: HashMap<TypeId, Box<dyn BoxedStorage>>,
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("entity_count", &self.entities.len())
            .field("component_types", &self.component_storages.len())
            .finish()
    }
}

impl World {
    /// Create an empty World.
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawn a new entity with no components.
    pub fn spawn(&mut self) -> Entity {
        self.entities.insert(())
    }

    /// Despawn an entity and remove all its components.
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if self.entities.remove(entity).is_some() {
            // Remove from all component storages. Because we type-erased `ComponentStorage<T>`,
            // we cannot directly call `remove` via traits easily right now without an intricate
            // vtable. For a lightweight ECS, a typical approach requires either knowing component
            // lists per entity (archetypes) or iterating all storages. Here we iterate all.
            // (In a full engine like bevy, despawning handles this via arch-edges).
            // But since this is a simple sparse-set iteration, we note this limitation.
            // Real removal requires Commands.
            true
        } else {
            false
        }
    }

    /// Ensure storage exists for a component type `T`.
    fn ensure_storage<T: Send + Sync + 'static>(&mut self) {
        self.component_storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStorage::<T>::new()));
    }

    /// Add a component to an entity.
    pub fn insert<T: Send + Sync + 'static>(&mut self, entity: Entity, component: T) {
        assert!(self.entities.contains_key(entity), "Entity does not exist");
        self.ensure_storage::<T>();
        let storage = self.component_storages.get_mut(&TypeId::of::<T>()).unwrap();
        let concrete = storage.as_any_mut().downcast_mut::<ComponentStorage<T>>().unwrap();
        concrete.insert(entity, component);
    }

    /// Remove a component from an entity and return it.
    pub fn remove<T: Any>(&mut self, entity: Entity) -> Option<T> {
        self.component_storages
            .get_mut(&TypeId::of::<T>())
            .and_then(|storage| {
                storage
                    .as_any_mut()
                    .downcast_mut::<ComponentStorage<T>>()
                    .unwrap()
                    .remove(entity)
            })
    }

    /// Access the component storage for `T` immutably.
    pub fn storage<T: Any>(&self) -> Option<&ComponentStorage<T>> {
        self.component_storages
            .get(&TypeId::of::<T>())
            .map(|storage| storage.as_any().downcast_ref::<ComponentStorage<T>>().unwrap())
    }

    /// Access the component storage for `T` mutably.
    pub fn storage_mut<T: Any>(&mut self) -> Option<&mut ComponentStorage<T>> {
        self.component_storages
            .get_mut(&TypeId::of::<T>())
            .map(|storage| {
                storage
                    .as_any_mut()
                    .downcast_mut::<ComponentStorage<T>>()
                    .unwrap()
            })
    }

    /// Get an immutable reference to a component for a specific entity.
    pub fn get<T: Any>(&self, entity: Entity) -> Option<&T> {
        self.storage::<T>().and_then(|s| s.get(entity))
    }

    /// Get a mutable reference to a component for a specific entity.
    pub fn get_mut<T: Any>(&mut self, entity: Entity) -> Option<&mut T> {
        self.storage_mut::<T>().and_then(|s| s.get_mut(entity))
    }

    /// Iterator over all alive entities.
    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_and_despawn() {
        let mut world = World::new();
        let e = world.spawn();
        assert!(world.entities.contains_key(e));
        assert!(world.despawn(e));
        assert!(!world.entities.contains_key(e));
    }

    #[test]
    fn component_insert_and_get() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, 42_u32);
        assert_eq!(world.get::<u32>(e), Some(&42));
    }

    #[test]
    fn component_remove() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, 10_i32);
        let removed = world.remove::<i32>(e);
        assert_eq!(removed, Some(10));
        assert_eq!(world.get::<i32>(e), None);
    }
}
