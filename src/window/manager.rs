//! Window creation and management using winit.

use anyhow::Result;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Fullscreen, Window, WindowBuilder},
};

use crate::core::config::Config;

/// Manages the OS window and exposes the raw handle for wgpu surface creation.
#[derive(Debug)]
pub struct WindowManager {
    /// The winit window.
    pub window: Window,
}

impl WindowManager {
    /// Create a new window from engine configuration.
    ///
    /// # Errors
    /// Returns an error if the OS refuses to create the window.
    pub fn new(config: &Config, event_loop: &EventLoop<()>) -> Result<Self> {
        let mut builder = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .with_resizable(true);

        if config.fullscreen {
            builder = builder.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        let window = builder
            .build(event_loop)
            .map_err(|e| anyhow::anyhow!("Failed to create window: {e}"))?;

        log::info!(
            "Window created: '{}' {}×{}",
            config.title,
            config.width,
            config.height
        );

        Ok(Self { window })
    }

    /// Physical size of the window's client area in pixels.
    pub fn size(&self) -> (u32, u32) {
        let s = self.window.inner_size();
        (s.width, s.height)
    }

    /// Set the window title at runtime.
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    /// Toggle fullscreen mode.
    pub fn set_fullscreen(&self, enabled: bool) {
        if enabled {
            self.window
                .set_fullscreen(Some(Fullscreen::Borderless(None)));
        } else {
            self.window.set_fullscreen(None);
        }
    }

    /// Request a redraw from the OS.
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    /// Expose the raw window handle required for wgpu surface creation.
    pub fn raw_window_handle(
        &self,
    ) -> &Window {
        &self.window
    }
}
