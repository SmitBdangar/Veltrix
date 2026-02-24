//! Renderer: wgpu device, sprite batching, camera, textures, shaders, and color.

pub mod color;
pub mod device;
pub mod pipeline;
pub mod shader;
pub mod sprite_batcher;
pub mod texture;

pub use color::Color;
pub use device::RenderDevice;
pub use sprite_batcher::SpriteBatcher;
pub use texture::{Texture, TextureHandle};
