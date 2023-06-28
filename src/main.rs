pub mod vulkan;
pub mod window;

use vulkan::*;
use vulkano::device::QueueFlags;
use window::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

fn main() {
    // Vulkan Instance
    let instance = Vulkan::create_instance();

    // Event loop and surface for an interface between vulkan and window.
    let (event_loop, surface) = Window::create_window(&instance);

    // GPU selected to be used.
    let physical_device = Vulkan::create_device(instance.clone(), surface.clone()).unwrap();

    // Abstraction to the physical device and the queue family indexes.
    let (device, queue) = Vulkan::get_queue_families(physical_device.clone(), QueueFlags::GRAPHICS);

    // Swapchain to manage the way images are displayed, and array of images to be displayed.
    let (swap_chain, images) = Vulkan::create_swapchain(physical_device, surface, device.clone());

    // Used to store info of the framebuffer, and other subpasses to be used while rendering
    let render_pass = Vulkan::get_render_pass(device.clone(), &swap_chain);

    // Destination for rendering.
    let frame_buffers = Vulkan::get_framebuffers(&images, &render_pass);

    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        _ => (),
    });
}
