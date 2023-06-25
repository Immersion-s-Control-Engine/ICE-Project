pub mod window;
pub mod vulkan;

use vulkano::device::QueueFlags;
use window::*;
use vulkan::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

fn main() {
    let instance = Vulkan::create_instance();
    let (event_loop, surface) = Window::create_window(&instance);
    let physical_device =  Vulkan::create_device(instance.clone(), surface.clone()).unwrap();
    let (device, queue) = Vulkan::get_queue_families(physical_device.clone(), QueueFlags::GRAPHICS);
    let (swap_chain, images) = Vulkan::create_swapchain(physical_device, surface, device);
    event_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });
}