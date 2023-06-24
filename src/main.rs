pub mod window;
pub mod vulkan;

use window::*;
use vulkan::*;

fn main() {
    let instance = Vulkan::create_instance();
    let device =  Vulkan::create_device(instance).unwrap();
    Window::create_window();
}