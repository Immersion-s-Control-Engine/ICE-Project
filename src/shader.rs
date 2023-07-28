use std::sync::Arc;

use wgpu::{self, *};

pub fn get_shaders(device: Arc<Device>) -> Arc<ShaderModule> {
    let shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: Some("Shader module"),
        source: ShaderSource::Wgsl(include_str!("./Shaders/shader.wgsl").into()),
    });
    Arc::new(shader)
}
