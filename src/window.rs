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

// simple_logger::init_with_level(Level::max());
// let event_loop = EventLoop::new();

// let builder = WindowBuilder::new();
// let window = builder.build(&event_loop).unwrap();

// let mut windows: HashMap<WindowId, Window> = HashMap::new();
// windows.insert(window.id(), window);

// event_loop.run(move |event, window_target, control_flow| {
//     *control_flow = ControlFlow::Wait;

//     match event {
//         Event::WindowEvent {
//             event: WindowEvent::CloseRequested,
//             ..
//         } => {
//             println!("The close button was pressed; stopping");
//             *control_flow = ControlFlow::Exit
//         }
//         Event::WindowEvent {
//             event: WindowEvent::KeyboardInput { input, .. },
//             ..
//         } => {
//             if let Some(VirtualKeyCode::N) = input.virtual_keycode {
//                 if let ElementState::Pressed = input.state {
//                     // println!("{:?}", input);
//                     let window = Window::new(&window_target).unwrap();
//                     windows.insert(window.id(), window);
//                 }
//             }
//             // keyboard_action(input, window_target, &mut windows);
//         }
//         Event::MainEventsCleared => {
//             // window.request_redraw();
//         }
//         _ => {}
//     }
// });
// }

// fn spawn_new_window(win_target: &EventLoopWindowTarget<()>) {
//     println!("Trying to create window");
//     let window = WindowBuilder::new()
//         .with_inner_size(LogicalSize::new(640, 640))
//         .with_visible(true)
//         .build(win_target)
//         .unwrap();
//     println!("{:?}", window.id());
//     // windows.insert(window.id(), window);
// }

// fn keyboard_action(
//     input: KeyboardInput,
//     win_target: &EventLoopWindowTarget<()>,
//     windows: &HashMap<WindowId, Window>,
// ) {
//     println!("{:?}", input);
//     match input.virtual_keycode {
//         Some(VirtualKeyCode::N) => spawn_new_window(win_target),
//         _ => println!("Wrong key to create window"),
//     }
// }
