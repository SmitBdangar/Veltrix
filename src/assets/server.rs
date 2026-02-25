//! The `AssetServer` provides asynchronous/synchronous loading of assets and caching.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use super::cache::AssetCache;
use super::handle::{AssetId, Handle};

/// Common trait for all loadable asset types.
pub trait Asset: Send + Sync + 'static {
    /// Load the asset from bytes.
    fn load(bytes: &[u8], ext: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}

/// The main entry point for loading and fetching assets.
#[derive(Clone)]
pub struct AssetServer {
    /// Thread-safe asset cache.
    cache: Arc<Mutex<AssetCache>>,
    /// Maps file paths to their canonical AssetId.
    paths: Arc<Mutex<HashMap<PathBuf, AssetId>>>,
    /// Fallback static bundled byte arrays (e.g. from `include_bytes!`).
    embedded_assets: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
    /// Global monotonic ID generator.
    next_id: Arc<Mutex<u64>>,
    /// Asset root folder (e.g., `assets/`).
    root: PathBuf,
}

impl AssetServer {
    /// Create a new asset server pointing to the given root directory.
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            cache: Arc::new(Mutex::new(AssetCache::new())),
            paths: Arc::new(Mutex::new(HashMap::new())),
            embedded_assets: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            root: root.as_ref().to_path_buf(),
        }
    }

    /// Load an asset synchronously from disk and cache it.
    ///
    /// pub 
    /// If the asset is already loaded, returns a cloned handle without hitting disk.
    pub fn load<T: Asset>(&self, path: impl AsRef<Path>) -> anyhow::Result<Handle<T>> {
        let path = path.as_ref();
        let full_path = self.root.join(path);

        // Check if already loaded
        {
            let paths_guard = self.paths.lock().unwrap();
            if let Some(&id) = paths_guard.get(path) {
                return Ok(Handle::new(id));
            }
        }

        // Read bytes (check embedded assets first, then fallback to filesystem)
        let bytes = {
            let embedded = self.embedded_assets.lock().unwrap();
            if let Some(data) = embedded.get(path) {
                data.clone()
            } else {
                std::fs::read(&full_path)
                    .map_err(|e| anyhow::anyhow!("Failed to read asset {path:?}: {e}"))?
            }
        };
            
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        // Parse (this depends on the specific T implementation of Asset trait)
        let asset = T::load(&bytes, ext)?;

        // Allocate ID and cache
        let mut id_guard = self.next_id.lock().unwrap();
        let id_val = *id_guard;
        *id_guard += 1;
        let id = AssetId(id_val);

        self.paths.lock().unwrap().insert(path.to_path_buf(), id);
        self.cache.lock().unwrap().insert(id, asset);

        Ok(Handle::new(id))
    }

    /// Retrieve an immutably borrowed asset if it is strictly ready in the cache.
    ///
    /// The user provides a closure because the cache requires locking the Mutex.
    pub fn with_asset<T: 'static, R, F>(&self, handle: &Handle<T>, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        let cache = self.cache.lock().unwrap();
        cache.get::<T>(handle.id()).map(f)
    }

    /// Retrieve a mutably borrowed asset (useful for hot reload).
    pub fn with_asset_mut<T: 'static, R, F>(&self, handle: &Handle<T>, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut cache = self.cache.lock().unwrap();
        cache.get_mut::<T>(handle.id()).map(f)
    }

    /// Add an asset created manually at runtime (not loaded from disk).
    pub fn add<T: Send + Sync + 'static>(&self, asset: T) -> Handle<T> {
        let mut id_guard = self.next_id.lock().unwrap();
        let id_val = *id_guard;
        *id_guard += 1;
        let id = AssetId(id_val);

        self.cache.lock().unwrap().insert(id, asset);
        Handle::new(id)
    }

    /// Embed an asset's raw bytes into the server. If this path is requested via `load`,
    /// the server will use these bytes instead of attempting to read from the OS filesystem.
    pub fn embed_asset(&self, path: impl AsRef<Path>, bytes: Vec<u8>) {
        self.embedded_assets
            .lock()
            .unwrap()
            .insert(path.as_ref().to_path_buf(), bytes);
    }
}
