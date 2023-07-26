use std::sync::Arc;

use wgpu::{self, *};
use winit::window::Window;

pub async fn get_config(
    adapter: Arc<Adapter>,
    surface: Arc<Surface>,
    window: Arc<Window>,
) -> (SurfaceConfiguration, Arc<TextureFormat>) {
    let size = window.inner_size();
    let format = surface.get_preferred_format(&adapter).unwrap();
    let config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    (config, Arc::new(format))
}
