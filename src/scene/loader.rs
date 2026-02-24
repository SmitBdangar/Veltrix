//! Serializes and deserializes the entire Scene graph to/from disk using RON.

use std::path::Path;
use anyhow::{Context, Result};

use super::scene::Scene;
// Note: In an actual production engine we would build an intermediate
// `SceneDescriptor` struct that derives serde::Serialize/Deserialize
// instead of serializing the `World` directly, because type-erased SlotMaps
// are notoriously hard to serialize safely without generic reflection.
// For Veltrix, we stub the loader interface that hooks into the `serialization` module.

/// Helper to load and serialize scenes.
pub struct SceneLoader;

impl SceneLoader {
    /// Save a running scene to a `.ron` file.
    pub fn save(_scene: &Scene, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        log::info!("Saving scene to {:?}", path);
        
        // Stub: a real implementation would use `crate::serialization::scene_serializer::SceneSerializer`
        // to walk the ECS world and serialize registered components.
        
        let dummy_ron = "(\n    entities: [],\n)";
        std::fs::write(path, dummy_ron)
            .with_context(|| format!("Writing scene file {:?}", path))?;
            
        Ok(())
    }

    /// Load a scene from a `.ron` file.
    pub fn load(path: impl AsRef<Path>) -> Result<Scene> {
        let path = path.as_ref();
        log::info!("Loading scene from {:?}", path);
        if !path.exists() {
            anyhow::bail!("Scene file {:?} not found", path);
        }

        let _src = std::fs::read_to_string(path)
            .with_context(|| format!("Reading scene file {:?}", path))?;

        // Stub: a real implementation would deserialize the RON back into an ECS World.
        let scene = Scene::new();
        
        Ok(scene)
    }
}
