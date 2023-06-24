pub mod Vulkan {
    // Just a recap, Arc is used to create clones to the same memory address.
    use std::sync::Arc;
    use vulkano::device::physical::PhysicalDevice;
    use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
    use vulkano::instance::{Instance, InstanceCreateInfo, InstanceExtensions};
    use vulkano::VulkanLibrary;

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

    // This function is used to create a physical device.
    pub fn create_device(instance: Arc<Instance>) -> Option<Arc<PhysicalDevice>> {
        for device in instance.enumerate_physical_devices().unwrap() {
            if device.properties().device_name.contains("GeForce") {
                return Some(device);
            }
        }
        None
    }

    // This device is used to get queue families based on the type of queue required.
    pub fn get_queue_families(
        physical_device: Arc<PhysicalDevice>,
        queue_flag:QueueFlags
    ) -> (Arc<Device>, impl ExactSizeIterator) {
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, queue_family_property)| {
                queue_family_property
                    .queue_flags
                    .contains(queue_flag)
            })
            .expect("Couldn't find a graphical queue family")
            as u32;
            
            // Device in this case is a logical device.
        let (device, queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");
        (device, queues)
    }
}
