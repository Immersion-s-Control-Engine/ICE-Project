#![allow(dead_code)]
use crate::vertex_data::sphere_data;
use bytemuck::{cast_slice, Pod, Zeroable};
use cgmath::{ortho, perspective, Matrix4, Point3, Rad, Vector3};
use std::{f32::consts::PI, mem, sync::Arc};
use wgpu::{self, util::DeviceExt, *};

const ANIMATION_SPEED: f32 = 1.0;
const IS_PERSPECTIVE: bool = true;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Light {
    color: [f32; 4],
    specular_color: [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

pub fn light(c: [f32; 3], sc: [f32; 3], ai: f32, di: f32, si: f32, ss: f32) -> Light {
    Light {
        color: [c[0], c[1], c[2], 1.0],
        specular_color: [sc[0], sc[1], sc[2], 1.0],
        ambient_intensity: ai,
        diffuse_intensity: di,
        specular_intensity: si,
        specular_shininess: ss,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
}

#[allow(dead_code)]
pub fn vertex(p: [f32; 3], n: [f32; 3]) -> Vertex {
    Vertex {
        position: [p[0], p[1], p[2], 1.0],
        normal: [n[0], n[1], n[2], 1.0],
    }
}

#[rustfmt::skip]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub fn get_render_pipeline(
    device: Arc<Device>,
    shader: Arc<ShaderModule>,
    queue: Arc<Queue>,
    config: &SurfaceConfiguration,
    light_data: Light,
) -> (
    Arc<RenderPipeline>,
    Buffer,
    BindGroup,
    Buffer,
    Matrix4<f32>,
    Matrix4<f32>,
    u32,
) {
    let camera_position = (3.0, 1.5, 3.0).into();
    let look_direction = (0.0, 0.0, 0.0).into();
    let up_direction = cgmath::Vector3::unit_y();

    let _model_mat = create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let (view_mat, project_mat, _) = create_view_projection(
        camera_position,
        look_direction,
        up_direction,
        config.width as f32 / config.height as f32,
        IS_PERSPECTIVE,
    );
    // create vertex uniform buffer
    // model_mat and view_projection_mat will be stored in vertex_uniform_buffer inside the update function
    let vertex_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex Uniform Buffer"),
        size: 192,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // create fragment uniform buffer. here we set eye_position = camera_position and light_position = eye_position
    let fragment_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Fragment Uniform Buffer"),
        size: 32,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // store light and eye positions
    let light_position: &[f32; 3] = camera_position.as_ref();
    let eye_position: &[f32; 3] = camera_position.as_ref();
    queue.write_buffer(
        &fragment_uniform_buffer,
        0,
        bytemuck::cast_slice(light_position),
    );
    queue.write_buffer(
        &fragment_uniform_buffer,
        16,
        bytemuck::cast_slice(eye_position),
    );

    // create light uniform buffer
    let light_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Light Uniform Buffer"),
        size: 48,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // store light parameters
    queue.write_buffer(
        &light_uniform_buffer,
        0,
        bytemuck::cast_slice(&[light_data]),
    );

    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Uniform Bind Group Layout"),
        });

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: vertex_uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: fragment_uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: light_uniform_buffer.as_entire_binding(),
            },
        ],
        label: Some("Uniform Bind Group"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&uniform_bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    let vertex_data = create_vertices(2.0, 15, 30);

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: cast_slice(&vertex_data),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let num_vertices = vertex_data.len() as u32;

    (
        Arc::new(pipeline),
        vertex_buffer,
        uniform_bind_group,
        vertex_uniform_buffer,
        view_mat,
        project_mat,
        num_vertices,
    )
}

/*fn create_vertices() -> Vec<Vertex> {
    let pos = vertex_data::cube_positions();
    let normal = vertex_data::cube_normals();
    let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i]));
    }
    data.to_vec()
}
Used for cubes
*/
fn create_vertices(r: f32, u: usize, v: usize) -> Vec<Vertex> {
    let (pos, normal, _uvs) = sphere_data(r, u, v);
    let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i]));
    }
    data.to_vec()
}

pub fn create_view(
    camera_position: Point3<f32>,
    look_direction: Point3<f32>,
    up_direction: Vector3<f32>,
) -> Matrix4<f32> {
    Matrix4::look_at_rh(camera_position, look_direction, up_direction)
}

pub fn create_projection(aspect: f32, is_perspective: bool) -> Matrix4<f32> {
    let project_mat: Matrix4<f32>;
    if is_perspective {
        project_mat = OPENGL_TO_WGPU_MATRIX * perspective(Rad(2.0 * PI / 5.0), aspect, 0.1, 100.0);
    } else {
        project_mat = OPENGL_TO_WGPU_MATRIX * ortho(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0);
    }
    project_mat
}
pub fn create_view_projection(
    camera_position: Point3<f32>,
    look_direction: Point3<f32>,
    up_direction: Vector3<f32>,
    aspect: f32,
    is_perspective: bool,
) -> (Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) {
    // construct view matrix
    let view_mat = Matrix4::look_at_rh(camera_position, look_direction, up_direction);

    // construct projection matrix
    let project_mat: Matrix4<f32>;
    if is_perspective {
        project_mat = OPENGL_TO_WGPU_MATRIX * perspective(Rad(2.0 * PI / 5.0), aspect, 0.1, 100.0);
    } else {
        project_mat = OPENGL_TO_WGPU_MATRIX * ortho(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0);
    }

    // contruct view-projection matrix
    let view_project_mat = project_mat * view_mat;

    // return various matrices
    (view_mat, project_mat, view_project_mat)
}

pub fn create_perspective_projection(
    fovy: Rad<f32>,
    aspect: f32,
    near: f32,
    far: f32,
) -> Matrix4<f32> {
    OPENGL_TO_WGPU_MATRIX * perspective(fovy, aspect, near, far)
}

pub fn create_projection_ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Matrix4<f32> {
    OPENGL_TO_WGPU_MATRIX * ortho(left, right, bottom, top, near, far)
}

pub fn create_view_projection_ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
    camera_position: Point3<f32>,
    look_direction: Point3<f32>,
    up_direction: Vector3<f32>,
) -> (Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) {
    // construct view matrix
    let view_mat = Matrix4::look_at_rh(camera_position, look_direction, up_direction);

    // construct projection matrix
    let project_mat = OPENGL_TO_WGPU_MATRIX * ortho(left, right, bottom, top, near, far);

    // contruct view-projection matrix
    let view_project_mat = project_mat * view_mat;

    // return various matrices
    (view_mat, project_mat, view_project_mat)
}

pub fn create_transforms(
    translation: [f32; 3],
    rotation: [f32; 3],
    scaling: [f32; 3],
) -> Matrix4<f32> {
    // create transformation matrices
    let trans_mat =
        Matrix4::from_translation(Vector3::new(translation[0], translation[1], translation[2]));
    let rotate_mat_x = Matrix4::from_angle_x(Rad(rotation[0]));
    let rotate_mat_y = Matrix4::from_angle_y(Rad(rotation[1]));
    let rotate_mat_z = Matrix4::from_angle_z(Rad(rotation[2]));
    let scale_mat = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);

    // combine all transformation matrices together to form a final transform matrix: model matrix
    let model_mat = trans_mat * rotate_mat_z * rotate_mat_y * rotate_mat_x * scale_mat;

    // return final model matrix
    model_mat
}
