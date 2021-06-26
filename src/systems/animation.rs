use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn animation_system(mut animation_component_vector: &mut RefMut<Vec<Option<Animation>>>, time_passed: u128) {
    let iterator = animation_component_vector.iter_mut().filter_map(|animation| Some(animation.as_mut()?));
    for animation in iterator {
        if animation.running && !animation.sprites.is_empty() {
            animation.time_since_last_frame += time_passed as u32;
            if animation.time_since_last_frame >= animation.time_per_frame_ms {
                animation.time_since_last_frame -= animation.time_per_frame_ms;
                animation.current_frame_index += 1;
                if animation.current_frame_index >= animation.sprites.len() {
                    animation.current_frame_index = 0;
                }
            }
        }
    }
}