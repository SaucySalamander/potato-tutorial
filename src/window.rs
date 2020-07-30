use std::collections::HashMap;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder, WindowId},
};

pub struct VulkanWindow {
    pub windows: HashMap<WindowId, Window>,
}

impl VulkanWindow {

    pub fn init(event_loop: &EventLoopWindowTarget<()>) -> VulkanWindow {
        let mut vulkan_window = VulkanWindow{
            windows: HashMap::new(),
        };
        let window = VulkanWindow::init_window(&vulkan_window, &event_loop, "origin");
        vulkan_window.windows.insert(window.id(), window);
        vulkan_window
    }

    fn init_window(&self, event_loop: &EventLoopWindowTarget<()>, name: &str) -> Window {
        WindowBuilder::new()
            .with_title(name)
            .with_inner_size(LogicalSize::new(800, 600))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    pub fn init_event_loop(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, event_loop, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, window_id } => {
                    if let WindowEvent::CloseRequested = event {
                        println!("Window {:?} has received the signal to close", window_id);
                        self.windows.remove(&window_id);
                        if self.windows.is_empty() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    if let WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        is_synthetic,
                        ..
                    } = event
                    {
                        //TODO abstract keyboard input logic
                        if state == ElementState::Released
                            && virtual_keycode == Some(VirtualKeyCode::N)
                            && !is_synthetic
                        {
                            let window = self.init_window(event_loop, "spawn");
                            self.windows.insert(window.id(), window);
                        }
                    }
                }
                Event::MainEventsCleared => {
                    for (.., window) in self.windows.iter() {
                        window.request_redraw();
                    }
                }
                _ => (),
            }
        })
    }
}
