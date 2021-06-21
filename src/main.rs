mod state;
mod texture;
use state::*;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};


fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgpu simple 2d thingy")
        .build(&event_loop)
        .unwrap();
    
    let mut state = futures::executor::block_on(State::new(&window));
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                state.update_and_render();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == window.id() && !state.handle_window_event(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged {new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput {input, .. } => {
                            // TODO: REMOVE LATER AND LET THE STATE HANDLE ALL INPUT
                            match input {
                                KeyboardInput {
                                    state: ElementState::Pressed, 
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    });
}
