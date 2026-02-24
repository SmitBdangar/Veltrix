//! Typed strong and weak references to dynamically loaded assets.

use std::any::Any;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;

/// A type-erased unique identifier for a loaded asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId(pub u64);

/// A shared handle linking an asset to its source file ID.
///
/// Veltrix handles are relatively simple: they wrap an `Arc` containing the asset ID,
/// making cloning cheap.
#[derive(Debug)]
pub struct Handle<T> {
    id: AssetId,
    _marker: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Handle<T> {
    /// Create a new handle for a given asset ID.
    pub fn new(id: AssetId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    /// The unique ID of the asset this handle references.
    pub fn id(&self) -> AssetId {
        self.id
    }

    /// Erase the type of this handle.
    pub fn untyped(&self) -> UntypedHandle {
        UntypedHandle { id: self.id }
    }
}

/// A type-erased handle to an asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntypedHandle {
    pub(crate) id: AssetId,
}

impl UntypedHandle {
    /// Attempt to cast back to a typed handle.
    /// Note: This does not verify that the target asset is actually of type `T`.
    pub fn typed<T>(&self) -> Handle<T> {
        Handle::new(self.id)
    }
    
    /// The unqiue ID.
    pub fn id(&self) -> AssetId {
        self.id
    }
}
