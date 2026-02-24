//! A type-erased container for loaded assets of any type.

use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::handle::AssetId;

/// A generic cache for storing dynamically loaded assets by their unique ID.
#[derive(Default)]
pub struct AssetCache {
    /// Maps TypeId -> (Maps AssetId -> Box<dyn Any>)
    storages: HashMap<TypeId, HashMap<AssetId, Box<dyn Any + Send + Sync>>>,
}

impl AssetCache {
    /// Create a new, empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an asset into the cache.
    pub fn insert<T: Send + Sync + 'static>(&mut self, id: AssetId, asset: T) {
        let storage = self
            .storages
            .entry(TypeId::of::<T>())
            .or_insert_with(HashMap::new);
        storage.insert(id, Box::new(asset));
    }

    /// Retrieve a reference to a cached asset.
    pub fn get<T: 'static>(&self, id: AssetId) -> Option<&T> {
        self.storages
            .get(&TypeId::of::<T>())
            .and_then(|storage| storage.get(&id))
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Retrieve a mutable reference to a cached asset (useful for hot-reloading).
    pub fn get_mut<T: 'static>(&mut self, id: AssetId) -> Option<&mut T> {
        self.storages
            .get_mut(&TypeId::of::<T>())
            .and_then(|storage| storage.get_mut(&id))
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Remove an asset from the cache.
    pub fn remove<T: 'static>(&mut self, id: AssetId) -> Option<T> {
        self.storages
            .get_mut(&TypeId::of::<T>())
            .and_then(|storage| storage.remove(&id))
            .and_then(|boxed| boxed.downcast::<T>().ok().map(|b| *b))
    }

    /// Remove all assets of type `T`.
    pub fn clear<T: 'static>(&mut self) {
        if let Some(storage) = self.storages.get_mut(&TypeId::of::<T>()) {
            storage.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Texture {
        width: u32,
    }

    #[test]
    fn insert_and_get() {
        let mut cache = AssetCache::new();
        let id = AssetId(1);
        cache.insert(id, Texture { width: 1024 });

        let tex = cache.get::<Texture>(id).unwrap();
        assert_eq!(tex.width, 1024);
    }
}
