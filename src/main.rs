pub mod config;
pub mod device;
pub mod instance;
pub mod pipeline;
pub mod shader;
pub mod vertex_data;
pub mod window;

use std::{iter, sync::Arc};

use cgmath::Matrix4;
use config::get_config;
use device::get_device;
use instance::get_instance;
use pipeline::{create_projection, get_render_pipeline};
use shader::get_shaders;
use wgpu::{Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration, TextureFormat};
use window::get_window;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

const IS_PERSPECTIVE: bool = true;

struct State {
    init: InitWgpu,
    pipeline: Arc<RenderPipeline>,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    model_mat: Matrix4<f32>,
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
}

struct InitWgpu {
    instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub config: SurfaceConfiguration,
    pub format: Arc<TextureFormat>,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl InitWgpu {
    async fn new(window: Arc<Window>) -> Self {
        let (instance, surface, adapter) = get_instance(window.clone()).await;
        let (device, queue) = get_device(adapter.clone()).await;
        let size = window.inner_size();
        let (config, format) = get_config(adapter.clone(), surface.clone(), window.clone()).await;
        Self {
            instance,
            surface,
            device,
            queue,
            config,
            format,
            size,
        }
    }
}

impl State {
    async fn new(window: Arc<Window>) -> Self {
        let init = InitWgpu::new(window).await;
        let shader = get_shaders(init.device.clone());

        // uniform data
        let (
            pipeline,
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            model_mat,
            view_mat,
            project_mat,
        ) = get_render_pipeline(init.device.clone(), shader.clone(), &init.config);

        Self {
            init,
            pipeline,
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            model_mat,
            view_mat,
            project_mat,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init
                .surface
                .configure(&self.init.device, &self.init.config);

            self.project_mat = create_projection(
                new_size.width as f32 / new_size.height as f32,
                IS_PERSPECTIVE,
            );
            let mvp_mat = self.project_mat * self.view_mat * self.model_mat;
            let mvp_ref: &[f32; 16] = mvp_mat.as_ref();
            self.init
                .queue
                .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mvp_ref));
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //let output = self.init.surface.get_current_frame()?.output;
        let output = self.init.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.247,
                            b: 0.314,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                //depth_stencil_attachment: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..36, 0..1);
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let (event_loop, window) = get_window();
    let mut state = State::new(window.clone()).await;

    event_loop.run(
        move |event, _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.init.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        },
    );
}
