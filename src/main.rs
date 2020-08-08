mod io;
mod vulkan;
mod window;

use winit::event_loop::EventLoop;
use vulkan::vulk_init::VulkanApiObjects;
use log::debug;

fn main() {
    simple_logger::init_by_env();
    
    let event_loop = EventLoop::new();      
    let vulkan_api_objects = VulkanApiObjects::init(&event_loop);
    debug!("Done with init");

    debug!("Starting event loop");
    vulkan_api_objects.init_event_loop(event_loop);
}
