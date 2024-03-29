use std::sync::Arc;

use wgpu::{self, *};

pub async fn get_device(adapter: Arc<Adapter>) -> (Arc<Device>, Arc<Queue>) {
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("Logical Device"),
                features: Features::empty(),
                limits: Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device!");
    (Arc::new(device), Arc::new(queue))
}
