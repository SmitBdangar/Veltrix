//! Main `Engine` struct and `EngineBuilder` — engine entry point.
//!
//! The engine owns all subsystems and drives the main event loop.

use anyhow::Result;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use super::{config::Config, event::EventBus, game_loop::GameLoop, time::Time};
use crate::ecs::{world::World, resources::Resources};

/// Builder for configuring the engine before starting the main loop.
///
/// # Example
/// ```no_run
/// use veltrix::core::engine::EngineBuilder;
///
/// let engine = EngineBuilder::new()
///     .with_title("My Game")
///     .with_resolution(1280, 720)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct EngineBuilder {
    config: Config,
    config_path: Option<std::path::PathBuf>,
}

impl EngineBuilder {
    /// Create a builder with default settings.
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            config_path: None,
        }
    }

    /// Load configuration from a `.ron` file, overriding programmatic defaults.
    pub fn with_config_file(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.config_path = Some(path.into());
        self
    }

    /// Override the entire configuration object.
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    /// Set the window title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    /// Set the initial window resolution.
    pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    /// Enable or disable vertical synchronization.
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.config.vsync = vsync;
        self
    }

    /// Set a frames-per-second cap (0 = uncapped).
    pub fn with_fps_cap(mut self, fps: u32) -> Self {
        self.config.fps_cap = fps;
        self
    }

    /// Override the fixed timestep (seconds). Default: `1.0 / 60.0`.
    pub fn with_fixed_timestep(mut self, dt: f64) -> Self {
        self.config.fixed_timestep = dt;
        self
    }

    /// Build the engine, initialising all subsystems.
    ///
    /// # Errors
    /// Returns an error if a config file was specified but cannot be parsed.
    pub fn build(mut self) -> Result<Engine> {
        // Merge file-based config if provided.
        if let Some(path) = self.config_path.take() {
            self.config = Config::from_file(path)?;
        }

        // Initialise logging.
        let _ = env_logger::Builder::new()
            .parse_filters(&self.config.log_filter)
            .try_init();

        log::info!(
            "Initialising Veltrix engine — '{}' ({}×{})",
            self.config.title,
            self.config.width,
            self.config.height
        );

        Ok(Engine {
            config: self.config.clone(),
            game_loop: GameLoop::new(self.config.fixed_timestep),
            time: Time::new(),
            event_bus: EventBus::new(),
            world: World::new(),
            resources: Resources::new(),
        })
    }
}

/// The top-level engine struct that owns all subsystems.
///
/// Create via [`EngineBuilder`], then call [`Engine::run`] to start the loop.
#[derive(Debug)]
pub struct Engine {
    /// Engine configuration.
    pub config: Config,
    /// Fixed-timestep game loop state.
    pub game_loop: GameLoop,
    /// Frame timing.
    pub time: Time,
    /// Engine-wide event bus.
    pub event_bus: EventBus,
    /// ECS World.
    pub world: World,
    /// ECS Resources.
    pub resources: Resources,
}

impl Engine {
    /// Entry point shortcut — equivalent to `EngineBuilder::new().build()`.
    pub fn new() -> Result<Self> {
        EngineBuilder::new().build()
    }

    /// Run the main game loop using the provided callbacks.
    ///
    /// - `on_start`   — called once before the loop begins.
    /// - `on_update`  — called every frame with real delta time.
    /// - `on_fixed`   — called every fixed timestep (physics, etc.).
    /// - `on_render`  — called every frame with the interpolation alpha.
    ///
    /// The loop exits when the window is closed or `on_update` returns `false`.
    pub fn run<Start, Update, Fixed, Render>(
        mut self,
        on_start: Start,
        mut on_update: Update,
        mut on_fixed: Fixed,
        mut on_render: Render,
    ) -> Result<()>
    where
        Start: FnOnce(&mut World, &mut Resources),
        Update: FnMut(&mut World, &mut Resources, f64) -> bool,
        Fixed: FnMut(&mut World, &mut Resources, f64),
        Render: FnMut(&mut World, &mut Resources, f64),
    {
        // Fire on_start before the loop begins.
        on_start(&mut self.world, &mut self.resources);

        let event_loop: EventLoop<()> = EventLoop::new()
            .map_err(|e| anyhow::anyhow!("EventLoop creation failed: {e}"))?;
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.set_control_flow(ControlFlow::Poll);

        let _should_exit = false;

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    log::info!("Window close requested — shutting down.");
                    elwt.exit();
                }

                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    // Resize handled by WindowManager / RenderDevice externally.
                }

                Event::AboutToWait => {
                    // ---- Frame start ----
                    let real_dt = self.game_loop.begin_frame();
                    self.time.tick();

                    // Fixed-timestep iterations.
                    while self.game_loop.step() {
                        on_fixed(&mut self.world, &mut self.resources, self.config.fixed_timestep);
                    }

                    // Variable-timestep update.
                    if !on_update(&mut self.world, &mut self.resources, real_dt) {
                        log::info!("on_update returned false — exiting.");
                        elwt.exit();
                        return;
                    }

                    // Render with interpolation alpha.
                    let alpha = self.game_loop.alpha();
                    on_render(&mut self.world, &mut self.resources, alpha);

                    // Flush events after all systems have run.
                    self.event_bus.flush();

                    // FPS cap (when vsync is disabled).
                    if self.config.fps_cap > 0 && !self.config.vsync {
                        let target = std::time::Duration::from_secs_f64(
                            1.0 / self.config.fps_cap as f64,
                        );
                        self.time.enforce_fps_cap(target);
                    }
                }

                _ => {}
            }
        })
        .map_err(|e| anyhow::anyhow!("Event loop error: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_applies_overrides() {
        let engine = EngineBuilder::new()
            .with_title("Test")
            .with_resolution(800, 600)
            .with_vsync(false)
            .with_fps_cap(120)
            .build()
            .unwrap();
        assert_eq!(engine.config.title, "Test");
        assert_eq!(engine.config.width, 800);
        assert_eq!(engine.config.height, 600);
        assert!(!engine.config.vsync);
        assert_eq!(engine.config.fps_cap, 120);
    }

    #[test]
    fn default_config_has_sane_values() {
        let engine = EngineBuilder::new().build().unwrap();
        assert!(engine.config.fixed_timestep > 0.0);
        assert!(engine.config.width > 0);
    }
}
