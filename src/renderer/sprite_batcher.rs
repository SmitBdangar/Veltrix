//! Batches sprites for drawing.

use crate::scene::components::Sprite;
use crate::math::Transform2D;
use crate::renderer::device::RenderDevice;
use wgpu::util::DeviceExt;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexInput {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub transform_col0: [f32; 4],
    pub transform_col1: [f32; 4],
    pub transform_col2: [f32; 4],
    pub transform_col3: [f32; 4],
}

impl VertexInput {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexInput>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { offset: 0,  shader_location: 0, format: wgpu::VertexFormat::Float32x2 }, // position
                wgpu::VertexAttribute { offset: 8,  shader_location: 1, format: wgpu::VertexFormat::Float32x2 }, // uv
                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 }, // color
                wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 }, // transform_col0
                wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32x4 }, // transform_col1
                wgpu::VertexAttribute { offset: 64, shader_location: 5, format: wgpu::VertexFormat::Float32x4 }, // transform_col2
                wgpu::VertexAttribute { offset: 80, shader_location: 6, format: wgpu::VertexFormat::Float32x4 }, // transform_col3
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawCall {
    pub z_index: f32,
    pub vertices: [VertexInput; 6],
}

/// Submits sprites for instanced rendering.
pub struct SpriteBatcher {
    draw_calls: Vec<DrawCall>,
    pub vertex_buffer: Option<wgpu::Buffer>,
}

impl SpriteBatcher {
    pub fn new() -> Self {
        Self {
            draw_calls: Vec::new(),
            vertex_buffer: None,
        }
    }

    /// Appends 6 vertices representing a quad, associated with a custom z-index
    pub fn push_quad(&mut self, vertices: [VertexInput; 6], z_index: f32) {
        self.draw_calls.push(DrawCall { z_index, vertices });
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, transform: &Transform2D, _z_index: f32) {
        let mat = transform.to_mat4();

        // Standard quad spanning [-0.5, 0.5]
        let quads = [
            ( [-0.5, -0.5], [0.0, 1.0] ), // Bottom Left
            ( [ 0.5, -0.5], [1.0, 1.0] ), // Bottom Right
            ( [-0.5,  0.5], [0.0, 0.0] ), // Top Left

            ( [ 0.5, -0.5], [1.0, 1.0] ), // Bottom Right
            ( [ 0.5,  0.5], [1.0, 0.0] ), // Top Right
            ( [-0.5,  0.5], [0.0, 0.0] ), // Top Left
        ];

        let col0 = mat.col(0).into();
        let col1 = mat.col(1).into();
        let col2 = mat.col(2).into();
        let col3 = mat.col(3).into();

        let mut quad_vertices = [VertexInput {
            position: [0.0, 0.0],
            uv: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
            transform_col0: [0.0; 4],
            transform_col1: [0.0; 4],
            transform_col2: [0.0; 4],
            transform_col3: [0.0; 4],
        }; 6];

        for (i, (pos, uv)) in quads.iter().enumerate() {
            quad_vertices[i] = VertexInput {
                position: *pos,
                uv: *uv,
                color: [sprite.color.r, sprite.color.g, sprite.color.b, sprite.color.a],
                transform_col0: col0,
                transform_col1: col1,
                transform_col2: col2,
                transform_col3: col3,
            };
        }

        self.push_quad(quad_vertices, _z_index);
    }

    pub fn flush<'a>(&'a mut self, device: &RenderDevice, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.draw_calls.is_empty() {
            return;
        }

        // Sort draw calls by z-index (lowest to highest) for back-to-front rendering
        self.draw_calls.sort_by(|a, b| a.z_index.partial_cmp(&b.z_index).unwrap_or(std::cmp::Ordering::Equal));

        let mut flattened_vertices = Vec::with_capacity(self.draw_calls.len() * 6);
        for call in &self.draw_calls {
            flattened_vertices.extend_from_slice(&call.vertices);
        }

        // Reallocate buffer if it's too small or missing
        let needed_size = (flattened_vertices.len() * mem::size_of::<VertexInput>()) as u64;
        let mut recreate = false;
        
        if let Some(buf) = &self.vertex_buffer {
            if buf.size() < needed_size {
                recreate = true;
            }
        } else {
            recreate = true;
        }

        if recreate {
            self.vertex_buffer = Some(device.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("SpriteBatcher Vertex Buffer"),
                size: needed_size.next_power_of_two(),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        if let Some(buf) = &self.vertex_buffer {
            device.queue.write_buffer(buf, 0, bytemuck::cast_slice(&flattened_vertices));
            render_pass.set_vertex_buffer(0, buf.slice(0..needed_size));
            render_pass.draw(0..(flattened_vertices.len() as u32), 0..1);
        }

        self.draw_calls.clear();
    }
}
