//! Engine configuration loaded from `engine.ron`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Overall engine configuration.
///
/// Loaded from `engine.ron` at startup; all fields have safe defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Window title shown in the title bar.
    pub title: String,
    /// Initial window width in logical pixels.
    pub width: u32,
    /// Initial window height in logical pixels.
    pub height: u32,
    /// Enable vertical synchronization.
    pub vsync: bool,
    /// Maximum frames per second when vsync is off (0 = uncapped).
    pub fps_cap: u32,
    /// Fixed physics/update timestep in seconds (default 1/60).
    pub fixed_timestep: f64,
    /// Start in fullscreen mode.
    pub fullscreen: bool,
    /// Enable MSAA (multi-sample anti-aliasing); 1 = disabled.
    pub msaa_samples: u32,
    /// Path to the initial scene file.
    pub initial_scene: Option<PathBuf>,
    /// Log filter string (e.g. `"info"`, `"veltrix=debug"`).
    pub log_filter: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "Veltrix Game".to_string(),
            width: 1280,
            height: 720,
            vsync: true,
            fps_cap: 0,
            fixed_timestep: 1.0 / 60.0,
            fullscreen: false,
            msaa_samples: 1,
            initial_scene: None,
            log_filter: "info".to_string(),
        }
    }
}

impl Config {
    /// Load configuration from a `.ron` file.
    ///
    /// Falls back to [`Config::default`] if the file does not exist.
    ///
    /// # Errors
    /// Returns an error if the file exists but cannot be parsed.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            log::info!("Config file {:?} not found — using defaults.", path);
            return Ok(Self::default());
        }
        let src = std::fs::read_to_string(path)
            .with_context(|| format!("Reading config file {:?}", path))?;
        ron::from_str(&src)
            .with_context(|| format!("Parsing config file {:?}", path))
    }

    /// Serialize the configuration and write it to a `.ron` file.
    ///
    /// # Errors
    /// Returns an error if the file cannot be created or written.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let serialized = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .context("Serializing config")?;
        std::fs::write(path, serialized)
            .with_context(|| format!("Writing config to {:?}", path))
    }

    /// Aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use tempfile::NamedTempFile;

    #[test]
    fn default_config_is_valid() {
        let cfg = Config::default();
        assert_eq!(cfg.width, 1280);
        assert_eq!(cfg.height, 720);
        assert!(cfg.vsync);
        assert!((cfg.aspect_ratio() - (16.0 / 9.0)).abs() < 0.01);
    }

    #[test]
    fn config_round_trips_through_ron() {
        let original = Config {
            title: "Test Game".to_string(),
            width: 800,
            height: 600,
            vsync: false,
            fps_cap: 120,
            ..Default::default()
        };
        let tmp = NamedTempFile::new().unwrap();
        original.save_to_file(tmp.path()).unwrap();
        let loaded = Config::from_file(tmp.path()).unwrap();
        assert_eq!(loaded.title, "Test Game");
        assert_eq!(loaded.width, 800);
        assert!(!loaded.vsync);
        assert_eq!(loaded.fps_cap, 120);
    }
}
