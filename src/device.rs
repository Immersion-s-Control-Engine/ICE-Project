use std::sync::Arc;

use wgpu::{self, *};
use winit::window::Window;

pub async fn get_device(adapter: Arc<Adapter>) -> (Arc<Device>, Arc<Queue>) {
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device!");
    (Arc::new(device), Arc::new(queue))
}

