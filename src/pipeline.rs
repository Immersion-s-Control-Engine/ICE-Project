use std::sync::Arc;

use wgpu::{self, *};

pub fn get_render_pipeline(
    device: Arc<Device>,
    shader: Arc<ShaderModule>,
    format: Arc<TextureFormat>,
) -> (Arc<PipelineLayout>, Arc<RenderPipeline>) {
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: shader.as_ref(),
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(FragmentState {
            module: shader.as_ref(),
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: *format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        multiview: None,
    });
    (Arc::new(pipeline_layout), Arc::new(render_pipeline))
}
