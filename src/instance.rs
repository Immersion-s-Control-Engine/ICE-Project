use std::sync::Arc;

use wgpu::{self, *};
use winit::window::Window;

pub async fn get_instance(window: Arc<Window>) -> (Arc<Instance>, Arc<Surface>, Arc<Adapter>) {
    let instance = Instance::new(Backends::VULKAN);
    let surface = unsafe { instance.create_surface::<Window>(window.as_ref()) };
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
