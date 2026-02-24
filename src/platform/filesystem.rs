//! Cross-platform file system helpers.

use std::path::{Path, PathBuf};
use directories::ProjectDirs;

/// Provides consistent paths for saving app data across Windows, macOS, and Linux.
pub struct FileSystem {
    app_id: String,
    dirs: Option<ProjectDirs>,
}

impl FileSystem {
    /// Create a new filesystem resolver for the given application ID.
    ///
    /// `app_id` should typically be formatted as `com.organization.appname`.
    pub fn new(app_id: &str) -> Self {
        let parts: Vec<&str> = app_id.split('.').collect();
        let dirs = if parts.len() >= 3 {
            ProjectDirs::from(parts[0], parts[1], parts[2])
        } else {
            ProjectDirs::from("", "", app_id)
        }
        .or_else(|| ProjectDirs::from("", "", "VeltrixApp"));

        Self {
            app_id: app_id.to_string(),
            dirs,
        }
    }

    /// Returns the ideal directory for storing user saves and persistent config.
    pub fn user_data_dir(&self) -> Option<PathBuf> {
        self.dirs.as_ref().map(|d: &ProjectDirs| d.data_dir().to_path_buf())
    }

    /// Returns the directory for temporary cache files.
    pub fn cache_dir(&self) -> Option<PathBuf> {
        self.dirs.as_ref().map(|d: &ProjectDirs| d.cache_dir().to_path_buf())
    }

    /// Recursively ensure a directory exists.
    pub fn ensure_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
        if !path.as_ref().exists() {
            std::fs::create_dir_all(path)?;
        }
        Ok(())
    }
}
