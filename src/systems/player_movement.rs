use std::cell::{RefCell, RefMut};
use crate::components::*;

const PLAYER_SPEED: f32 = 0.01;

pub fn player_movement_system(
    mut velocity_component_vector: &mut RefMut<Vec<Option<Velocity>>>, 
    player_index: usize, 
    keyboard_inputs_frame: &Vec<winit::event::KeyboardInput>
) {
    if let Some(Some(velocity)) = velocity_component_vector.get_mut(player_index) {
        for keyb_input in keyboard_inputs_frame {
            match keyb_input {
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::W),
                    ..
                } => {
                    velocity.vel_y = -PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::A),
                    ..
                } => {
                    velocity.vel_x = -PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::S),
                    ..
                } => {
                    velocity.vel_y = PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::D),
                    ..
                } => {
                    velocity.vel_x = PLAYER_SPEED;
                },
                _ => {}
            }
        }
    }
}