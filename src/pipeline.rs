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
    let mut primitive_type = "point-list";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        primitive_type = &args[1];
    }

    let mut topology = wgpu::PrimitiveTopology::PointList;
    let mut index_format = None;
    if primitive_type == "line-list" {
        topology = wgpu::PrimitiveTopology::LineList;
        index_format = None;
    } else if primitive_type == "line-strip" {
        topology = wgpu::PrimitiveTopology::LineStrip;
        index_format = Some(wgpu::IndexFormat::Uint32);
    } else if primitive_type == "triangle-strip" {
        topology = wgpu::PrimitiveTopology::TriangleStrip;
        index_format = Some(IndexFormat::Uint32);
    } else if primitive_type == "triangle-list" {
        topology = wgpu::PrimitiveTopology::TriangleList;
        index_format = Some(IndexFormat::Uint32);
    }
    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[ColorTargetState {
                format: *format,
                blend: Some(BlendState {
                    color: BlendComponent::REPLACE,
                    alpha: BlendComponent::REPLACE,
                }),
                write_mask: ColorWrites::ALL,
            }],
        }),
        primitive: PrimitiveState {
            topology: topology,
            strip_index_format: index_format,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: MultisampleState::default(),
        multiview: None,
    });

    (Arc::new(pipeline_layout), Arc::new(render_pipeline))
}
