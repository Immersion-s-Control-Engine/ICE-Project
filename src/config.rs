use std::sync::Arc;

use wgpu::{self, *};
use winit::window::Window;

pub async fn get_config(
    adapter: Arc<Adapter>,
    surface: Arc<Surface>,
    window: Arc<Window>,
) -> (SurfaceConfiguration, Arc<TextureFormat>) {
    let size = window.inner_size();
    let format = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap()
        .format;
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Mailbox,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: Default::default(),
    };
    (config, Arc::new(format))
}
