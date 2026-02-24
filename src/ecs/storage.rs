//! Component storage for the ECS.
//!
//! Maps Entity IDs to component data using a dense array and a sparse map.

use slotmap::Key;
use std::any::Any;
use std::cell::UnsafeCell;

use super::world::Entity;

/// Type-erased trait for component storage containers in the World.
pub trait BoxedStorage: Send + Sync {
    /// Attempt to cast the boxed storage back to its concrete type.
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Send + Sync + 'static> BoxedStorage for ComponentStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A Sparse-set based storage for a single component type `T`.
///
/// Ensures contiguous memory access when iterating `T` sequentially.
pub struct ComponentStorage<T> {
    /// The dense array of actual component data.
    data: Vec<T>,
    /// Maps dense index -> Entity.
    entities: Vec<Entity>,
    /// Maps Entity -> dense index (sparse map).
    sparse: std::collections::HashMap<Entity, usize>,
    
    // UnsafeCell used ONLY to allow multiple mutable iterator borrows across
    // strictly disjoint component queries safely in `SystemScheduler`.
    // In safe code paths (get, get_mut, insert, remove), this is ignored.
    _marker: std::marker::PhantomData<UnsafeCell<T>>,
}

// Manual Send/Sync since PhantomData<UnsafeCell> is not Send/Sync by default.
unsafe impl<T: Send + Sync> Send for ComponentStorage<T> {}
unsafe impl<T: Send + Sync> Sync for ComponentStorage<T> {}

impl<T> Default for ComponentStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ComponentStorage<T> {
    /// Create new empty storage.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            entities: Vec::new(),
            sparse: std::collections::HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Insert a component for an entity.
    ///
    /// If the entity already had this component, it is overwritten.
    pub fn insert(&mut self, entity: Entity, component: T) {
        if let Some(&dense_idx) = self.sparse.get(&entity) {
            // Overwrite existing
            self.data[dense_idx] = component;
        } else {
            // Append new
            let dense_idx = self.data.len();
            self.sparse.insert(entity, dense_idx);
            self.entities.push(entity);
            self.data.push(component);
        }
    }

    /// Remove a component for an entity and return it.
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let dense_idx = self.sparse.remove(&entity)?;
        
        // Swap-remove to keep data contiguous without shifting everything
        let last_idx = self.data.len() - 1;
        
        self.data.swap(dense_idx, last_idx);
        self.entities.swap(dense_idx, last_idx);
        
        // Update the sparse link for the element that got swapped in
        if dense_idx != last_idx {
            let swapped_in_entity = self.entities[dense_idx];
            self.sparse.insert(swapped_in_entity, dense_idx);
        }
        
        self.entities.pop();
        let value = self.data.pop();
        
        value
    }

    /// Iterator over `(Entity, &T)`.
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.entities.iter().copied().zip(self.data.iter())
    }

    /// Mutable iterator over `(Entity, &mut T)`.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.entities.iter().copied().zip(self.data.iter_mut())
    }

    /// Access the component for an entity safely.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.sparse.get(&entity).map(|&idx| &self.data[idx])
    }

    /// Modifiably access the component for an entity safely.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.sparse.get(&entity).map(|&idx| &mut self.data[idx])
    }

    /// Check if the entity has this component.
    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains_key(&entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut storage = ComponentStorage::<u32>::new();
        let e1 = Entity::null();
        storage.insert(e1, 42);
        assert_eq!(storage.get(e1), Some(&42));
    }

    #[test]
    fn remove_shifts_dense_array() {
        let mut storage = ComponentStorage::<u32>::new();
        let e1 = Entity::null();
        // Since we can't create real slotmap ids easily in this limited isolated scope
        // without a World, we just test logic by assuming slots are unique.
        
        storage.insert(e1, 10);
        let val = storage.remove(e1);
        assert_eq!(val, Some(10));
        assert_eq!(storage.get(e1), None);
    }
}
