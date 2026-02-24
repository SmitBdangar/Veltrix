//! File system watching and hot-reloading using the `notify` crate.

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

/// Listens for file changes in the assets directory.
pub struct AssetLoader {
    watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>,
    /// Root path being watched (e.g. "./assets").
    root_dir: PathBuf,
}

impl AssetLoader {
    /// Create a new asset loader watching exactly the `root_dir`.
    pub fn new(root_dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        let root = root_dir.as_ref().to_path_buf();
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            // Send the event to our receiver channel
            let _ = tx.send(res);
        })?;

        // Watch recursively if the directory exists.
        if root.exists() {
            watcher.watch(&root, RecursiveMode::Recursive)?;
            log::info!("AssetLoader watching: {:?}", root);
        } else {
            log::warn!("AssetLoader root does not exist: {:?}", root);
        }

        Ok(Self {
            watcher,
            rx,
            root_dir: root,
        })
    }

    /// Poll for any changed files this frame.
    ///
    /// Returns a list of all absolute file paths that were modified.
    pub fn poll_changes(&self) -> Vec<PathBuf> {
        let mut changed = Vec::new();
        while let Ok(res) = self.rx.try_recv() {
            match res {
                Ok(event) => {
                    // We only care about file modifications.
                    if event.kind.is_modify() {
                        for path in event.paths {
                            changed.push(path);
                        }
                    }
                }
                Err(e) => log::error!("AssetLoader watch error: {:?}", e),
            }
        }
        // Deduplicate
        changed.sort();
        changed.dedup();
        changed
    }
}
