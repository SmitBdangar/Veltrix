//! Pipeline wrapper.

use super::device::RenderDevice;

/// Wrapper around wgpu::RenderPipeline.
pub struct RenderPipeline {
    pub wgpu_pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(
        device: &RenderDevice,
        shader: &wgpu::ShaderModule,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        label: Option<&str>,
    ) -> Self {
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        let wgpu_pipeline = device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: vertex_layouts,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: device.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self { wgpu_pipeline }
    }
}
