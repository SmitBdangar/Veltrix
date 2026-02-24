//! Scene graph serialization to `.ron`.

use std::path::Path;
use anyhow::Result;

use crate::scene::scene::Scene;

/// Utility for serializing an entire `Scene` (ECS world context) to disk.
pub struct SceneSerializer;

impl SceneSerializer {
    /// Save the given scene to `path` in RON format.
    ///
    /// Note: Veltrix's type-erased slotmaps are not natively compatible with `serde`.
    /// A full implementation requires a component registry to iterate and serialize
    /// known component types. This serves as the engine's public hook for it.
    pub fn save(_scene: &Scene, path: impl AsRef<Path>) -> Result<()> {
        crate::scene::loader::SceneLoader::save(_scene, path)
    }

    /// Load a scene from `path` in RON format.
    pub fn load(path: impl AsRef<Path>) -> Result<Scene> {
        crate::scene::loader::SceneLoader::load(path)
    }
}
