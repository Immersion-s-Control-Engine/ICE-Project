use std::{borrow::Cow, sync::Arc};

use wgpu::{self, *};

pub fn get_shaders(device: Arc<Device>) -> Arc<ShaderModule> {
    let shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./Shaders/shader.wgsl"))),
    });
    Arc::new(shader)
}
