mod io;
mod vulkan;
mod window;

use window::VulkanWindow;
use winit::event_loop::EventLoop;
use vulkan::vulk_init::VulkanApiObjects;

fn main() {
    simple_logger::init().unwrap();
    
    let event_loop = EventLoop::new();
    let vulkan_window = VulkanWindow::init(&event_loop);
    
    //TODO temp select one window for surface and swapchain generation
    let window_map = vulkan_window.windows.iter().next();
    let window = window_map.unwrap().1;
    
    let _vulkan_api_objects = VulkanApiObjects::init(&window);

    vulkan_window.init_event_loop(event_loop);
}
