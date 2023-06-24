pub mod window;
pub mod vulkan;

use vulkano::device::QueueFlags;
use window::*;
use vulkan::*;

fn main() {
    let instance = Vulkan::create_instance();
    let device =  Vulkan::create_device(instance).unwrap();
    Vulkan::get_queue_families(device, QueueFlags::GRAPHICS);
    Window::create_window();
}