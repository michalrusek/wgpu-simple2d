use std::cell::{RefCell, RefMut};
use crate::components::*;

const PLAYER_SPEED: f32 = 0.01;

pub fn player_movement_system(
    mut position_component_vector: &mut RefMut<Vec<Option<Position>>>, 
    player_index: usize, 
    keyboard_inputs_frame: &Vec<winit::event::KeyboardInput>
) {
    if let Some(Some(position)) = position_component_vector.get_mut(player_index) {
        for keyb_input in keyboard_inputs_frame {
            match keyb_input {
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::W),
                    ..
                } => {
                    position.y -= PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::A),
                    ..
                } => {
                    position.x -= PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::S),
                    ..
                } => {
                    position.y += PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::D),
                    ..
                } => {
                    position.x += PLAYER_SPEED;
                },
                _ => {}
            }
        }
    }
}