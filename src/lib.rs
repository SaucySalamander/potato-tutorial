mod io;
mod vulkan;

use log::debug;
use simple_logger::SimpleLogger;
use vulkan::vulk_init::VulkanApiObjects;
use winit::event_loop::EventLoop;

pub fn init_graphics() {
    SimpleLogger::new().env().init().unwrap();

    debug!("Init event_loop");
    let event_loop = EventLoop::new();
    debug!("Init vulkan api objects");
    let vulkan_api_objects = VulkanApiObjects::init(&event_loop);
    debug!("Done with init");

    debug!("Starting event loop");
    vulkan_api_objects.init_event_loop(event_loop);
}

pub fn init_compute() {
    SimpleLogger::new().env().init().unwrap();

    debug!("Init vulkan api objects");
    let vulkan_api_objects = VulkanApiObjects::init_compute();
}
