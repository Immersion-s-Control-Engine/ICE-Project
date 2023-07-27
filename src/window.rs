use std::{path::Path, sync::Arc};

use winit::{
    event_loop::EventLoop,
    window::{Icon, Window},
};

pub fn get_window() -> (EventLoop<()>, Arc<Window>) {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let icon = load_icon(Path::new("./ice_icon.png"));
    window.set_window_icon(Some(icon));
    window.set_title("ICE");
    env_logger::init();
    let window = Arc::new(window);
    (event_loop, window)
}

fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
