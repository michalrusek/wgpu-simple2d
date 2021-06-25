use std::cell::{RefCell, RefMut};
use crate::components::*;

const PLAYER_SPEED: f32 = 0.01;

pub fn player_movement_system(mut sprite_component_vector: &mut RefMut<Vec<Option<Sprite>>>, player_index: usize, keyboard_inputs_frame: &Vec<winit::event::KeyboardInput>) {
    if let Some(Some(sprite_component)) = sprite_component_vector.get_mut(player_index) {
        for keyb_input in keyboard_inputs_frame {
            match keyb_input {
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::W),
                    ..
                } => {
                    sprite_component.p1.1 -= PLAYER_SPEED;
                    sprite_component.p2.1 -= PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::A),
                    ..
                } => {
                    sprite_component.p1.0 -= PLAYER_SPEED;
                    sprite_component.p2.0 -= PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::S),
                    ..
                } => {
                    sprite_component.p1.1 += PLAYER_SPEED;
                    sprite_component.p2.1 += PLAYER_SPEED;
                },
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed, 
                    virtual_keycode: Some(winit::event::VirtualKeyCode::D),
                    ..
                } => {
                    sprite_component.p1.0 += PLAYER_SPEED;
                    sprite_component.p2.0 += PLAYER_SPEED;
                },
                _ => {}
            }
        }
    }
}