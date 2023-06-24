pub mod Window {
    use std::sync::Arc;

    use glfw::{Action, Key, WindowHint, ClientApiHint, Context};
    use vulkano::VulkanLibrary;

    pub fn create_window() {
    // This initiates glfw.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    // This creates an instance of a window in glfw.
    let (mut window, events) = glfw.create_window(300, 300, "ICE", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    //Setting the window hints.
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));

    // Used to register key actions and give feedback to window.
    window.set_key_polling(true);


    window.make_current();

    // This sets a loop to keep the window open.
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
    }

    // Gets the required extensions to work with Vulkan API.
    pub fn get_extensions() -> Vec<String> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.get_required_instance_extensions().unwrap()
    }

    pub fn check_supported() {
        let library: Arc<VulkanLibrary> = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        for (extension, boolean) in library.supported_extensions().into_iter() {
            println!("{} : {}", extension,boolean);
        }
    }
}

