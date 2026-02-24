//! GPU Texture loading.

use crate::assets::server::Asset;

/// Represents a loaded wgpu Texture on the GPU.
#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
}

impl Asset for Texture {
    fn load(_bytes: &[u8], _ext: &str) -> anyhow::Result<Self> {
        Ok(Self { width: 1, height: 1 })
    }
}

pub type TextureHandle = crate::assets::handle::Handle<Texture>;
