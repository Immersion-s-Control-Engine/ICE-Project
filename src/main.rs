pub mod config;
pub mod device;
pub mod instance;
pub mod pipeline;
pub mod shader;
pub mod window;

use config::get_config;
use device::get_device;
use instance::get_instance;
use pipeline::get_render_pipeline;
use shader::get_shaders;
use window::get_window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

#[tokio::main]
async fn main() {
    let (event_loop, window) = get_window();
    let (instance, surface, adapter) = get_instance(window.clone()).await;
    let (device, queue) = get_device(adapter.clone()).await;
    let (mut config, format) = get_config(adapter.clone(), surface.clone(), window.clone()).await;
    surface.configure(device.as_ref(), &config);
    let shader = get_shaders(device.clone());
    let (pipeline_layout, render_pipeline) =
        get_render_pipeline(device.clone(), shader.clone(), format.clone());

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &shader, &pipeline_layout);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the surface with the new size
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
            }
            Event::RedrawRequested(_) => {
                let frame = surface.get_current_texture().unwrap();
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.05,
                                    g: 0.062,
                                    b: 0.08,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
