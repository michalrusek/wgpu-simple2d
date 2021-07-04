use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn animation_system(
    animation_component_vector_opt: &mut Option<RefMut<Vec<Option<Animation>>>>, 
    animation_map_component_vector_opt: &mut Option<RefMut<Vec<Option<AnimationMap>>>>,
    time_passed: u128
) {
    // Simple animations
    {
        if let Some(animation_component_vector) = animation_component_vector_opt {
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
    }
    
    // Animation maps
    {
        if let Some(animation_map_component_vector) = animation_map_component_vector_opt {
            let iterator = animation_map_component_vector.iter_mut().filter_map(|animation_map| Some(animation_map.as_mut()?));
            for animation_map in iterator {
                if let Some(mut animation) = animation_map.map.get_mut(animation_map.current_animation_name) {
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
        }
    }
}