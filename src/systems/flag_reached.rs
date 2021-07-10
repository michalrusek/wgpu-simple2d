use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn flag_reached_system(
    collision_list_component_vector: &RefMut<Vec<Option<CollisionList>>>,  
    entity_type_component_vector: &RefMut<Vec<Option<EntityType>>>,  
    player_index: Option<usize>,
) -> bool {
    if let Some(player_index) = player_index {
        if let Some(Some(collision_list)) = collision_list_component_vector.get(player_index) {
            for collision in collision_list.list.iter() {
                if let Some(Some(entity_type)) = entity_type_component_vector.get(collision.collided_with) {
                    match entity_type {
                        &EntityType::EndFlag => {
                            return true;
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    
    false
}