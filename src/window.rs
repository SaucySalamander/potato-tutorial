use std::collections::HashMap;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowId, WindowBuilder},
};

pub fn init() {
    simple_logger::init().unwrap();
    //TODO Add surface init
    init_event_loop(EventLoop::new());
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

fn init_base_win(event_loop: &EventLoop<()>) -> HashMap<WindowId, Window> {
    let mut hm = HashMap::new();
    let window = spawn_win(event_loop, "origin");
    hm.insert(window.id(), window);
    hm
}

fn spawn_win(event_loop: &EventLoopWindowTarget<()>, name: &str) -> Window{
    WindowBuilder::new().with_title(name).build(event_loop).unwrap()
}