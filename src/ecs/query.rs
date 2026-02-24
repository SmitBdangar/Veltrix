//! Query builder for iterating entities with specific component sets.
//!
//! Because `World` uses type-erased component storages, true safe, zero-cost
//! complex queries (like `bevy` or `hecs`) require advanced macro or type-level
//! magic. For simplicity in Veltrix, we provide a basic `Query` struct that
//! allows extracting components dynamically at iteration time or via simple matching.

use super::world::{Entity, World};

/// A simple dynamic query to iterate entities holding a primary component type `T`,
/// optionally checking or fetching secondary components.
pub struct Query<'a, T> {
    world: &'a World,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Send + Sync + 'static> Query<'a, T> {
    /// Create a query for the given World, using `T` as the primary driver.
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            _marker: std::marker::PhantomData,
        }
    }

    /// Iterator over `(Entity, &T)`.
    pub fn iter(&self) -> Box<dyn Iterator<Item = (Entity, &T)> + '_> {
        if let Some(storage) = self.world.storage::<T>() {
            Box::new(storage.iter())
        } else {
            Box::new(std::iter::empty())
        }
    }
}

/// A mutable dynamic query. Requires borrowing the entire World mutably in this simple ECS.
pub struct QueryMut<'w, T> {
    world: &'w mut World,
    _marker: std::marker::PhantomData<T>,
}

impl<'w, T: Send + Sync + 'static> QueryMut<'w, T> {
    /// Create a mutable query.
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            _marker: std::marker::PhantomData,
        }
    }

    /// Iterator over `(Entity, &mut T)`.
    /// Note: cannot cleanly yield `&mut U` simultaneously in this simple model without unsafe cell iteration.
    pub fn iter_mut(&mut self) -> Box<dyn Iterator<Item = (Entity, &mut T)> + '_> {
        if let Some(storage) = self.world.storage_mut::<T>() {
            Box::new(storage.iter_mut())
        } else {
            Box::new(std::iter::empty())
        }
    }
}

// ── Utility macros for multi-component queries ───────────────────────────────
// In a production engine, this is where we'd implement tuples `(Entity, &A, &mut B)`
// using unsafe cell aliasing checks. For this implementation, we use simple getters
// inside the system loop on the World directly to fetch secondary components.

#[cfg(test)]
mod tests {
    use super::*;

    struct Position(f32, f32);
    struct Velocity(f32, f32);

    #[test]
    fn query_iterates_primary() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        
        world.insert(e1, Position(1.0, 2.0));
        world.insert(e2, Position(3.0, 4.0));
        
        let q = Query::<Position>::new(&world);
        let count = q.iter().count();
        assert_eq!(count, 2);
    }
}
