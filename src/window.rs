pub mod Window {
    use std::sync::Arc;
    use vulkano::{instance::Instance, swapchain::Surface};
    use vulkano_win::VkSurfaceBuild;
    use winit::{event_loop::EventLoop, window::{WindowBuilder}, dpi::Size};

    pub fn create_window(instance: &Arc<Instance>) -> (EventLoop<()>, Arc<Surface>) {
        let event_loop = EventLoop::new();  // ignore this for now
        let surface = WindowBuilder::new()
        .with_inner_size(Size::Physical(winit::dpi::PhysicalSize { width: 600, height: 600 }))
        .with_resizable(false)
        .with_title("ICE")
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();
        (event_loop, surface)
    }

    // Gets the required extensions to work with Vulkan API.
    pub fn get_extensions() -> Vec<String> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.get_required_instance_extensions().unwrap()
    }
}

