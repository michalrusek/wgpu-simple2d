use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn points_ticking_down(
    mut points_component_vector: &mut RefMut<Vec<Option<Points>>>,  
    time_passed: u128,
    player_index: Option<usize>,
) -> bool {
    if let Some(player_index) = player_index {
        if let Some(Some(player_points)) = points_component_vector.get_mut(player_index) {
    
            if player_points.points == 0 { return true; }
    
            player_points.time_since_last_point_change_ms += time_passed as u32;
            if player_points.time_since_last_point_change_ms >= 1000 {
                player_points.points -= 1;
                player_points.time_since_last_point_change_ms -= 1000;
            }
        }
    }
    
    false
}