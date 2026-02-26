//! Query builder for iterating entities with specific component sets.
//! Now powered by `hecs` for true zero-cost multi-component archetype queries.

use super::world::{Entity, World};

/// A dynamic query to iterate entities. 
/// Q should be a `hecs::Query` type like `(&Pos, &Vel)` or `(&mut Transform, &Sprite)`.
pub struct Query<'a, Q> {
    world: &'a World,
    _marker: std::marker::PhantomData<Q>,
}

impl<'a, Q: hecs::Query> Query<'a, Q> {
    /// Create a query for the given World.
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the iterator over `(Entity, Q::Item)`.
    pub fn iter(&self) -> hecs::QueryBorrow<'_, Q> {
        self.world.inner().query::<Q>()
    }
}

/// A mutable dynamic query. 
/// Q should be a `hecs::Query` type like `(&mut Pos, &Vel)`.
pub struct QueryMut<'w, Q> {
    world: &'w mut World,
    _marker: std::marker::PhantomData<Q>,
}

impl<'w, Q: hecs::Query> QueryMut<'w, Q> {
    /// Create a mutable query.
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the iterator over `(Entity, Q::Item)`.
    pub fn iter_mut(&mut self) -> hecs::QueryMut<'_, Q> {
        self.world.inner_mut().query_mut::<Q>()
    }
}
