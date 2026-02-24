//! Serialize user progress and save files.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// General-purpose key-value store for saving user progression, high scores,
/// unlocked levels, and inventory data.
///
/// Uses RON format.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SaveGame {
    pub string_vars: HashMap<String, String>,
    pub int_vars: HashMap<String, i64>,
    pub float_vars: HashMap<String, f64>,
    pub bool_vars: HashMap<String, bool>,
}

impl SaveGame {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a save file from disk.
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            log::info!("Save file not found, creating new at {:?}", path);
            return Ok(Self::new());
        }

        let data = std::fs::read_to_string(path)
            .with_context(|| format!("Reading save game at {:?}", path))?;
        
        let save_game: Self = ron::from_str(&data)
            .with_context(|| format!("Parsing save game RON at {:?}", path))?;

        log::info!("Save game loaded from {:?}", path);
        Ok(save_game)
    }

    /// Write current save states to disk.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        
        let pretty = ron::ser::PrettyConfig::default();
        let ron_data = ron::ser::to_string_pretty(self, pretty)?;

        std::fs::write(path, ron_data)
            .with_context(|| format!("Writing save game to {:?}", path))?;
            
        log::info!("Save game written to {:?}", path);
        Ok(())
    }
}
