//! Global resource storage for the ECS.
//!
//! Resources are singletons (e.g., `Time`, `InputManager`, `AssetServer`)
//! accessible by any system without being attached to an entity.

use anyhow::{anyhow, Result};
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A container for application-wide singletons.
#[derive(Default)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl std::fmt::Debug for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resources")
            .field("count", &self.data.len())
            .finish()
    }
}

impl Resources {
    /// Create an empty resource container.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a new resource, overwriting the old one if it exists.
    pub fn insert<T: Any + 'static>(&mut self, resource: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(resource));
    }

    /// Remove a resource of type `T` and return it.
    pub fn remove<T: Any + 'static>(&mut self) -> Option<T> {
        self.data
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast::<T>().ok().map(|b| *b))
    }

    /// Get an immutable reference to a resource of type `T`.
    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Get a mutable reference to a resource of type `T`.
    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.data
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Helper to unwrap `get` or return a nice `anyhow::Error`.
    pub fn expect<T: Any + 'static>(&self) -> Result<&T> {
        self.get::<T>()
            .ok_or_else(|| anyhow!("Resource {} not found", std::any::type_name::<T>()))
    }

    /// Helper to unwrap `get_mut` or return a nice `anyhow::Error`.
    pub fn expect_mut<T: Any + 'static>(&mut self) -> Result<&mut T> {
        self.get_mut::<T>()
            .ok_or_else(|| anyhow!("Resource {} not found", std::any::type_name::<T>()))
    }

    /// Returns `true` if the resource exists.
    pub fn contains<T: Any + 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Time {
        dt: f32,
    }

    #[test]
    fn insert_and_get() {
        let mut r = Resources::new();
        r.insert(Time { dt: 0.1 });
        
        let t = r.get::<Time>().unwrap();
        assert_eq!(t.dt, 0.1);
    }

    #[test]
    fn get_mut_modifies() {
        let mut r = Resources::new();
        r.insert(Time { dt: 0.1 });
        
        r.get_mut::<Time>().unwrap().dt = 0.5;
        
        assert_eq!(r.get::<Time>().unwrap().dt, 0.5);
    }

    #[test]
    fn remove_deletes() {
        let mut r = Resources::new();
        r.insert(Time { dt: 0.1 });
        
        assert!(r.contains::<Time>());
        r.remove::<Time>();
        assert!(!r.contains::<Time>());
    }
}
