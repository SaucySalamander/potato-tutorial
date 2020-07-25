mod io;
mod vulkan;
mod window;

//use crate::io::file::{read_file_to_str, write_file, append_file};
use vulkan::vulk_init::{create_instance, create_physical_device, setup_device, create_swapchain};
use window::{init_windows, spawn_win};
use winit::event_loop::EventLoop;
use winit::window::{Window};

use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::QueueFamily;
use vulkano::swapchain::{Surface};
use std::sync::Arc;
use vulkano::device::{Device, QueuesIter, Queue};


fn main() {
    // let _ = read_file_to_str("./config.yaml".as_ref());
    // let _ = write_file("./test.txt".as_ref(), "".as_ref());
    // let _ = append_file("./test.txt".as_ref(), "this is a test".as_ref());
    let event_loop = EventLoop::new();
    init_building_blocks(&event_loop);
    init_windows(event_loop);
}

fn init_building_blocks(event_loop: &EventLoop<()>){
    let surface = spawn_win(event_loop, "init");
    let physical_device = create_physical_device(surface.instance());
    let queue_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
        .unwrap();
    let (device, mut queues) = setup_device(&physical_device, &queue_family);

    let queue = queues.next().unwrap();

    create_swapchain(&surface, &physical_device);
}
