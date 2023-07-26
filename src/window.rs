use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

pub fn get_window() -> (EventLoop<()>, Arc<Window>) {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("ICE");
    env_logger::init();
    let window = Arc::new(window);
    (event_loop, window)
}
