//! Render device wrapper.

use std::sync::Arc;
use winit::window::Window;

/// Wrapper around wgpu::Device and Queue, managing the swapchain surface.
pub struct RenderDevice {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,
}

impl RenderDevice {
    /// Creates a new RenderDevice asynchronously via block_on.
    pub fn new(window: Arc<Window>) -> Self {
        pollster::block_on(Self::new_async(window.clone()))
    }

    async fn new_async(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // The surface needs to live as long as the window that created it.
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("RenderDevice"),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // We favor sRGB surfaces
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
            
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoNoVsync, // Defaulted to off for maximum performance test
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        }
    }

    /// Resize the WGPU Surface when the window resizes
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Start a frame, acquiring the surface texture.
    pub fn begin_frame(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    /// Set the window title at runtime.
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    /// Set whether the OS cursor is visible. Useful for custom sprite cursors.
    pub fn set_cursor_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible);
    }

    /// Set the OS cursor icon (e.g. pointer, text).
    pub fn set_cursor_icon(&self, icon: winit::window::CursorIcon) {
        self.window.set_cursor_icon(icon);
    }

    /// Set the OS window icon from RGBA bytes.
    pub fn set_window_icon(&self, rgba: Vec<u8>, width: u32, height: u32) {
        if let Ok(icon) = winit::window::Icon::from_rgba(rgba, width, height) {
            self.window.set_window_icon(Some(icon));
        } else {
            log::warn!("Failed to create window icon from RGBA data.");
        }
    }
}
