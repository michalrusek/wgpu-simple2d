use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn player_pineapple_system(
    collision_list_component_vector: &RefMut<Vec<Option<CollisionList>>>, 
    mut marked_for_deletion_component_vector: &mut RefMut<Vec<Option<MarkedForDeletion>>>, 
    entity_type_component_vector: &RefMut<Vec<Option<EntityType>>>, 
    mut points_component_vector: &mut RefMut<Vec<Option<Points>>>, 
    player_index: Option<usize>
) {
    if let Some(player_index) = player_index {
        if let (
            Some(Some(collision_list)), 
            Some(Some(points))
        ) = (
            collision_list_component_vector.get(player_index), 
            points_component_vector.get_mut(player_index)
        ) {
            for collision in collision_list.list.iter() {
                if let Some(Some(entity_type)) = entity_type_component_vector.get(collision.collided_with) {
                    match entity_type {
                        &EntityType::Pineapple => {
                            points.points += 1;
                            if let Some(Some(marked_for_deletion)) = marked_for_deletion_component_vector.get_mut(collision.collided_with) {
                                marked_for_deletion.marked = true;
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}