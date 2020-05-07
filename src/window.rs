use std::collections::HashMap;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub fn init() {
    simple_logger::init().unwrap();
    let event_loop = EventLoop::new();

    let mut windows = HashMap::new();
    let window = Window::new(&event_loop).unwrap();
    windows.insert(window.id(), window);

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
            }
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::Key(KeyboardInput {
                    virtual_keycode,
                    state,
                    ..
                }) = event
                {
                    if state == ElementState::Released && virtual_keycode == Some(VirtualKeyCode::N)
                    {
                        let window = Window::new(&event_loop).unwrap();
                        windows.insert(window.id(), window);
                    }
                }
            }
            Event::MainEventsCleared => {
                for (.., window) in windows.iter() {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    })
}
