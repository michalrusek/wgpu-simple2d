use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn player_animation_system(
    mut velocity_component_vector: &mut RefMut<Vec<Option<Velocity>>>, 
    mut animation_map_component_vector: &mut RefMut<Vec<Option<AnimationMap>>>, 
    player_index: usize, 
) {
    if let (
        Some(Some(velocity)),
        Some(Some(animation_map)),
    ) = (
        velocity_component_vector.get(player_index),
        animation_map_component_vector.get_mut(player_index),
    ) {
        let ensure_correct_animation_playing = |animation_map: &mut AnimationMap, animation_name: &str| {
            if animation_map.current_animation_name != animation_name {
                disable_current_animation(animation_map);
                enable_animation(animation_map, animation_name);
            }
        };

        let (vel_x, vel_y) = (velocity.vel_x, velocity.vel_y);

        if vel_y < 0. {
            ensure_correct_animation_playing(animation_map, "jump");
            if vel_x < 0. {
                animation_map.horiz_mirror = true;
            } else {
                animation_map.horiz_mirror = false;
            }
        } else if vel_y > 0. {
            ensure_correct_animation_playing(animation_map, "fall");
            if vel_x < 0. {
                animation_map.horiz_mirror = true;
            } else {
                animation_map.horiz_mirror = false;
            }
        } else if vel_x > 0. {
            ensure_correct_animation_playing(animation_map, "running_right");
            animation_map.horiz_mirror = false;
        } else if vel_x < 0. {
            ensure_correct_animation_playing(animation_map, "running_right");
            animation_map.horiz_mirror = true;
        } else if vel_y  == 0. {
            ensure_correct_animation_playing(animation_map, "idle");
        } 
    }
}

fn disable_current_animation (animation_map: &mut AnimationMap) {
    if let Some(mut animation) = animation_map.map.get_mut(animation_map.current_animation_name) {
        animation.running = false;
        animation.current_frame_index = 0;
        animation.time_since_last_frame = 0;
    }
}
fn enable_animation (animation_map: &mut AnimationMap, new_animation_name: &str) {
    if let Some(mut animation) = animation_map.map.get_mut(new_animation_name) {
        animation.running = true;
        animation.current_frame_index = 0;
        animation.time_since_last_frame = 0;
        animation_map.current_animation_name = animation.animation_name;
    }
}
