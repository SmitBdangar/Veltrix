//! Text rendering via ab_glyph and wgpu sprite batching.

use crate::renderer::Color;
use crate::assets::handle::Handle;

/// A component that renders a string of text.
#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub font: Handle<FontAsset>,
    pub font_size: f32,
    pub color: Color,
}

impl Text {
    pub fn new(text: impl Into<String>, font: Handle<FontAsset>, font_size: f32) -> Self {
        Self {
            text: text.into(),
            font,
            font_size,
            color: Color::WHITE,
        }
    }
}

/// A loaded TrueType/OpenType font.
#[derive(Debug, Clone)]
pub struct FontAsset {
    pub data: Vec<u8>,
}

impl crate::assets::server::Asset for FontAsset {
    fn load(bytes: &[u8], _ext: &str) -> anyhow::Result<Self> {
        Ok(Self {
            data: bytes.to_vec(),
        })
    }
}

pub struct FontAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
    /// Maps ascii char (offset by 32) to (U, V, U_Width, V_Height)
    pub uvs: [(f32, f32, f32, f32); 95],
    pub glyph_widths: [f32; 95],
    pub line_height: f32,
}

impl FontAtlas {
    /// Creates a static grid atlas for ASCII characters 32 to 126.
    pub fn new(
        rd: &crate::renderer::RenderDevice,
        bind_group_layout: &wgpu::BindGroupLayout,
        font_data: &[u8],
        font_size: f32,
    ) -> Self {
        use ab_glyph::{Font, FontRef, ScaleFont};
        let font = FontRef::try_from_slice(font_data).unwrap();
        let scale = ab_glyph::PxScale::from(font_size);
        let scaled = font.as_scaled(scale);

        let grid_cols = 10;
        let grid_rows = 10;
        let cell_size = (font_size * 1.5) as usize; // generous bounding box
        let tex_width = grid_cols * cell_size;
        let tex_height = grid_rows * cell_size;

        let mut pixels = vec![0u8; (tex_width * tex_height * 4) as usize];
        let mut uvs = [(0.0, 0.0, 0.0, 0.0); 95];
        let mut widths = [0.0; 95];

        for i in 0..95 {
            let c = (i as u8 + 32) as char;
            let glyph = scaled.scaled_glyph(c);
            let h_advance = scaled.h_advance(glyph.id);
            widths[i] = h_advance;

            if let Some(outlined) = scaled.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let col = i % grid_cols;
                let row = i / grid_cols;

                let px = col * cell_size;
                let py = row * cell_size;

                outlined.draw(|x, y, coverage| {
                    let map_x = px + x as usize;
                    let map_y = py + y as usize;
                    if map_x < tex_width && map_y < tex_height {
                        let idx = (map_y * tex_width + map_x) * 4;
                        let alpha = (coverage * 255.0) as u8;
                        pixels[idx] = 255;
                        pixels[idx + 1] = 255;
                        pixels[idx + 2] = 255;
                        pixels[idx + 3] = alpha;
                    }
                });

                let w = bounds.width();
                let h = bounds.height();
                uvs[i as usize] = (
                    px as f32 / tex_width as f32,
                    py as f32 / tex_height as f32,
                    w / tex_width as f32,
                    h / tex_height as f32,
                );
            }
        }

        let texture_size = wgpu::Extent3d {
            width: tex_width as u32,
            height: tex_height as u32,
            depth_or_array_layers: 1,
        };

        let texture = rd.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("ASCII Font Atlas"),
            view_formats: &[],
        });

        rd.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some((4 * tex_width) as u32),
                rows_per_image: Some(tex_height as u32),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = rd.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = rd.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Font Atlas Bind Group"),
        });

        Self {
            texture,
            view,
            sampler,
            bind_group,
            uvs,
            glyph_widths: widths,
            line_height: scaled.height(),
        }
    }
}

pub struct TextBatcher;

impl TextBatcher {
    pub fn draw_text(
        batcher: &mut crate::renderer::SpriteBatcher,
        text: &str,
        atlas: &FontAtlas,
        transform: &crate::math::Transform2D,
        color: crate::renderer::Color,
    ) {
        let mut cursor_x = 0.0;
        let mut cursor_y = 0.0;
        
        // Base transform
        let base_mat = transform.to_mat4();

        for c in text.chars() {
            if c == '\n' {
                cursor_y -= atlas.line_height;
                cursor_x = 0.0;
                continue;
            }

            let ascii = c as usize;
            if ascii < 32 || ascii > 126 {
                continue; // unsupported char
            }

            let idx = ascii - 32;
            let (u, v, uw, vh) = atlas.uvs[idx];
            
            // Width/height in pixel coordinate space for rendering
            let width = uw * atlas.texture.width() as f32;
            let height = vh * atlas.texture.height() as f32;

            // Character transform relative to the base transform.
            // Adjust vertical alignment so the text is vertically anchored nicely.
            let char_offset = crate::math::Vec3::new(
                cursor_x + width * 0.5, 
                cursor_y + height * 0.5 - atlas.line_height * 0.8,
                0.0
            );

            let local_mat = crate::math::Mat4::from_scale_rotation_translation(
                crate::math::Vec3::new(width, height, 1.0),
                crate::math::Quat::IDENTITY,
                char_offset,
            );

            let final_mat = base_mat * local_mat;

            // Extract columns for instance data
            let col0 = final_mat.col(0).into();
            let col1 = final_mat.col(1).into();
            let col2 = final_mat.col(2).into();
            let col3 = final_mat.col(3).into();

            // Push a standard centered quad with custom UVs
            let quads = [
                ( [-0.5, -0.5], [u, v + vh] ), 
                ( [ 0.5, -0.5], [u + uw, v + vh] ), 
                ( [-0.5,  0.5], [u, v] ), 
                ( [ 0.5, -0.5], [u + uw, v + vh] ),
                ( [ 0.5,  0.5], [u + uw, v] ),
                ( [-0.5,  0.5], [u, v] ),
            ];

            let mut quad_verts = [crate::renderer::sprite_batcher::VertexInput {
                position: [0.0, 0.0],
                uv: [0.0, 0.0],
                color: [0.0; 4],
                transform_col0: [0.0; 4],
                transform_col1: [0.0; 4],
                transform_col2: [0.0; 4],
                transform_col3: [0.0; 4],
            }; 6];

            for (i, (pos, uv)) in quads.iter().enumerate() {
                quad_verts[i] = crate::renderer::sprite_batcher::VertexInput {
                    position: *pos,
                    uv: *uv,
                    color: [color.r, color.g, color.b, color.a],
                    transform_col0: col0,
                    transform_col1: col1,
                    transform_col2: col2,
                    transform_col3: col3,
                };
            }

            batcher.push_quad(quad_verts, 50.0); // Text renders on UI layer

            cursor_x += atlas.glyph_widths[idx];
        }
    }
}
