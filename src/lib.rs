pub mod config;
pub mod device;
pub mod instance;
pub mod pipeline;
pub mod shader;
pub mod vertex_data;
pub mod window;
use cgmath::Matrix4;
use cgmath::{Matrix, SquareMatrix};
use config::get_config;
use device::get_device;
use instance::get_instance;
use pipeline::{create_projection, create_transforms, get_render_pipeline, light};
use shader::get_shaders;
use std::iter;
use std::sync::Arc;
use wgpu;
use wgpu::{BindGroup, Buffer, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::{event::WindowEvent, window::Window};

const IS_PERSPECTIVE: bool = true;
const ANIMATION_SPEED: f32 = 1.0;
pub struct State {
    pub init: InitWgpu,
    pipeline: Arc<RenderPipeline>,
    vertex_buffer: Buffer,
    uniform_bind_group: BindGroup,
    vertex_uniform_buffer: Buffer,
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
    num_vertices: u32,
}

pub struct InitWgpu {
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl InitWgpu {
    async fn new(window: Arc<Window>) -> Self {
        let (surface, adapter) = get_instance(window.clone()).await;
        let (device, queue) = get_device(adapter.clone()).await;
        let size = window.inner_size();
        let config = get_config(adapter.clone(), surface.clone(), window.clone()).await;
        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let init = InitWgpu::new(window).await;
        let shader = get_shaders(init.device.clone());
        let light_data = light([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], 0.1, 0.6, 0.3, 30.0);

        // uniform data
        let (
            pipeline,
            vertex_buffer,
            uniform_bind_group,
            vertex_uniform_buffer,
            view_mat,
            project_mat,
            num_vertices,
        ) = get_render_pipeline(
            init.device.clone(),
            shader.clone(),
            init.queue.clone(),
            &init.config,
            light_data,
        );

        Self {
            init,
            pipeline,
            vertex_buffer,
            uniform_bind_group,
            vertex_uniform_buffer,
            view_mat,
            project_mat,
            num_vertices,
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
        }
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        // update uniform buffer
        let dt = ANIMATION_SPEED * dt.as_secs_f32();
        let model_mat =
            create_transforms([0.0, 0.0, 0.0], [dt.sin(), dt.cos(), 0.0], [1.0, 1.0, 1.0]);
        let view_project_mat = self.project_mat * self.view_mat;

        let normal_mat = (model_mat.invert().unwrap()).transpose();

        let model_ref: &[f32; 16] = model_mat.as_ref();
        let view_projection_ref: &[f32; 16] = view_project_mat.as_ref();
        let normal_ref: &[f32; 16] = normal_mat.as_ref();

        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            0,
            bytemuck::cast_slice(model_ref),
        );
        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            64,
            bytemuck::cast_slice(view_projection_ref),
        );
        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            128,
            bytemuck::cast_slice(normal_ref),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
