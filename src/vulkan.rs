pub mod Vulkan {
    // Just a recap, Arc is used to create clones to the same memory address.
    use std::sync::Arc;
    use vulkano::device::physical::PhysicalDevice;
    use vulkano::device::{
        Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags,
    };
    use vulkano::image::sys::ImageCreateInfo;
    use vulkano::image::view::ImageView;
    use vulkano::image::{ImageUsage, SwapchainImage};
    use vulkano::instance::{Instance, InstanceCreateInfo, InstanceExtensions};
    use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
    use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo};
    use vulkano::VulkanLibrary;
    use winit::window::Window;

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
        let extensions: InstanceExtensions = vulkano_win::required_extensions(&library);
        println!("{:?}", extensions);
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
    pub fn create_device(
        instance: Arc<Instance>,
        surface: Arc<Surface>,
    ) -> Option<Arc<PhysicalDevice>> {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        for device in instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    // Find the first first queue family that is suitable.
                    // If none is found, `None` is returned to `filter_map`,
                    // which disqualifies this physical device.
                    .position(|(i, q)| {
                        q.queue_flags.contains(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            })
        {
            if device.0.properties().device_name.contains("GeForce") {
                return Some(device.0);
            }
        }
        None
    }

    // This device is used to get queue families based on the type of queue required.
    pub fn get_queue_families(
        physical_device: Arc<PhysicalDevice>,
        queue_flag: QueueFlags,
    ) -> (Arc<Device>, impl ExactSizeIterator) {
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, queue_family_property)| {
                queue_family_property.queue_flags.contains(queue_flag)
            })
            .expect("Couldn't find a graphical queue family")
            as u32;

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        // Device in this case is a logical device.
        let (device, queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create device");
        (device, queues)
    }

    pub fn create_swapchain(
        physical_device: Arc<PhysicalDevice>,
        surface: Arc<Surface>,
        device: Arc<Device>,
    ) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
        let caps = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("failed to get surface capabilities");
        let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
        let dimensions: [u32; 2] = window.inner_size().into();
        let composite_alpha: vulkano::swapchain::CompositeAlpha =
            caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format: Option<vulkano::format::Format> = Some(
            physical_device
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0,
        );
        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
                image_format,
                present_mode: vulkano::swapchain::PresentMode::Fifo,
                image_extent: dimensions,
                image_color_space: vulkano::swapchain::ColorSpace::SrgbNonLinear,
                image_usage: ImageUsage::COLOR_ATTACHMENT, // What the images are going to be used for
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap();

        (swapchain, images)
    }

    pub fn get_render_pass(device: Arc<Device>, swapchain: &Arc<Swapchain>) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.image_format(), // set the format the same as the swapchain
                    samples: 1,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .unwrap()
    }

    pub fn get_framebuffers(
        images: &Vec<Arc<SwapchainImage>>,
        render_pass: &Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }
}
