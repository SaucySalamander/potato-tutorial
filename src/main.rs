mod io;
mod vulkan;

use winit::event_loop::EventLoop;
use vulkan::vulk_init::VulkanApiObjects;
use log::debug;

fn main() {
    simple_logger::init_by_env();
    
    debug!("Init event_loop");
    let event_loop = EventLoop::new(); 
    debug!("Init vulkan api objects");
    let vulkan_api_objects = VulkanApiObjects::init(&event_loop);
    debug!("Done with init");

    debug!("Starting event loop");
    vulkan_api_objects.init_event_loop(event_loop);
}
