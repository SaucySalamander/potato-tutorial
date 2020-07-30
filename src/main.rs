mod io;
mod vulkan;
mod window;

use window::VulkanWindow;
use winit::event_loop::EventLoop;
use vulkan::vulk_init::VulkanApiObjects;
use log::{info};

fn main() {
    simple_logger::init().unwrap();
    let _vulkan_api_objects = VulkanApiObjects::init();

    let event_loop = EventLoop::new();
    let vulkan_window = VulkanWindow::init(&event_loop);

    vulkan_window.init_event_loop(event_loop);
}
