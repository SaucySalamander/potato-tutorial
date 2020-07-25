#[path = "vulkan/mod.rs"]
mod vulkan;
use vulkan::vulk_init::create_instance;
use vulkano_win::VkSurfaceBuild;
use std::collections::HashMap;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowId, WindowBuilder},
};
use vulkano::swapchain::{Surface};
use std::sync::Arc;
use vulkano::instance::Instance;


pub fn init_windows(event_loop: EventLoop<()>) {
    simple_logger::init().unwrap();
    init_event_loop(event_loop);
}

fn init_event_loop<>(event_loop: EventLoop<()>){
    
    let mut windows = init_base_win(&event_loop);

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } => {
                if let WindowEvent::CloseRequested = event {
                    println!("Window {:?} has received the signal to close", window_id);
                    windows.remove(&window_id);
                    if windows.is_empty() {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                if let WindowEvent::KeyboardInput{input: KeyboardInput{ virtual_keycode, state, ..}, is_synthetic,  ..} = event { //TODO abstract keyboard input logic
                    if state == ElementState::Released && virtual_keycode == Some(VirtualKeyCode::N) && !is_synthetic
                    {
                        let window = spawn_win(event_loop, "spawn");
                        windows.insert(window.window().id(), window);
                    }
                }
            }
            Event::MainEventsCleared => {
                for (.., window) in windows.iter() {
                    window.window().request_redraw();
                }
            }
            _ => (),
        }
    })
}

fn init_base_win(event_loop: &EventLoop<()>) -> HashMap<WindowId, Arc<Surface<Window>>> {
    let mut hm = HashMap::new();
    let window = spawn_win(event_loop, "origin");
    hm.insert(window.window().id(), window);
    hm
}

pub fn spawn_win(event_loop: &EventLoopWindowTarget<()>, name: &str) -> Arc<Surface<Window>>{
    let instance = create_instance();
    WindowBuilder::new().with_title(name).build_vk_surface(event_loop, instance).unwrap()
}