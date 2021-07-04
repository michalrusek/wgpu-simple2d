use std::cell::{RefCell, RefMut};
use crate::components::*;

const PLAYER_SPEED: f32 = 0.1;
const PLAYER_VERTICAL_SPEED: f32 = PLAYER_SPEED * 20.;
const PLAYER_HORIZONTAL_SPEED: f32 = PLAYER_SPEED * 5.;

pub fn player_movement_system(
    mut velocity_component_vector: &mut RefMut<Vec<Option<Velocity>>>, 
    player_index: usize, 
    keyboard_inputs_frame: &Vec<winit::event::KeyboardInput>
) {
    // TODO: Do a proper state machine here instead
    if let Some(Some(velocity)) = velocity_component_vector.get_mut(player_index) {
        for keyb_input in keyboard_inputs_frame {
            match keyb_input {
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::W),
                    ..
                } => {
                    if velocity.vel_y == 0. {
                        velocity.vel_y = -PLAYER_VERTICAL_SPEED;
                    }
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::A),
                    ..
                } => {
                    velocity.vel_x = -PLAYER_HORIZONTAL_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::D),
                    ..
                } => {
                    velocity.vel_x = PLAYER_HORIZONTAL_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Released, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::A),
                    ..
                } => {
                    if velocity.vel_x < 0. {
                        velocity.vel_x = 0.;
                    }
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Released, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::D),
                    ..
                } => {
                    if velocity.vel_x > 0. {
                        velocity.vel_x = 0.;
                    }
                },
                _ => {}
            }
        }
    }
}