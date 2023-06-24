pub mod Vulkan {
    // Just a recap, Arc is used to create clones to the same memory address.
    use std::sync::Arc;
    use vulkano::device::physical::PhysicalDevice;
    use vulkano::instance::{Instance, InstanceCreateInfo, InstanceExtensions};
    use vulkano::{VulkanLibrary};

    use crate::window::Window;

    // An instance stores all of the applications state info.
    pub fn create_instance() -> Arc<Instance> {
        let library: Arc<VulkanLibrary> =
            VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let layers: Vec<_> = library
            .layer_properties()
            .unwrap()
            .filter(|l| l.description().contains("Validation"))
            .collect();

        // Fetching extensions to be enabled.
        let extensions: InstanceExtensions =
            InstanceExtensions::from_iter(Window::get_extensions().iter().map(|x| x.as_str()));
        let instance: Arc<Instance> = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: extensions,
                enabled_layers: layers.iter().map(|l| l.name().to_owned()).collect(),
                ..Default::default()
            },
        )
        .expect("failed to create instance");
        instance
    }

    pub fn create_device(instance: Arc<Instance>) -> Option<Arc<PhysicalDevice>> {
        for device in instance.enumerate_physical_devices().unwrap() {
            if device.properties().device_name.contains("GeForce") {
                return Some(device);
            }
        }
        None
    }
}
