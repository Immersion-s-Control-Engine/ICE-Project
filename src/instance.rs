use std::sync::Arc;

use wgpu::{self, *};
use winit::window::Window;

pub async fn get_instance(window: Arc<Window>) -> (Arc<Instance>, Arc<Surface>, Arc<Adapter>) {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::VULKAN,
        dx12_shader_compiler: Dx12Compiler::default(),
    });
    let surface = unsafe { instance.create_surface(window.as_ref()).unwrap() };
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find appropriate Adapter!");
    (Arc::new(instance), Arc::new(surface), Arc::new(adapter))
}
