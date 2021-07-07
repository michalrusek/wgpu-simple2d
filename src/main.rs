mod renderer;
mod texture;
mod game;
mod components;
mod systems;
use game::Game;
use renderer::*;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

const DESIRED_RENDER_SIZE: [u32; 2] = [1280_u32, 720_u32];


fn main() {
    println!("Hello, world!");
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgpu simple 2d thingy")
        .with_inner_size(winit::dpi::PhysicalSize::new(DESIRED_RENDER_SIZE[0], DESIRED_RENDER_SIZE[1]))
        .with_resizable(false) // TODO: Remove once resizing is handled properly on the renderer
        .build(&event_loop)
        .unwrap();
    
    let mut renderer = futures::executor::block_on(Renderer::new(&window, winit::dpi::PhysicalSize::new(DESIRED_RENDER_SIZE[0], DESIRED_RENDER_SIZE[1])));
    let mut game = Game::new(DESIRED_RENDER_SIZE);
    game.init(&mut renderer);
    
    let mut last_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                let time_passed = last_time.elapsed().as_millis();
                last_time = std::time::Instant::now();

                game.update(time_passed);
                renderer.render(&game.get_renderables());
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged {new_inner_size, .. } => {
                            renderer.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput {input, .. } => {
                            game.process_keyboard_input(input);
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

