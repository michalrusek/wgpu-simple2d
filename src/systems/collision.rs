use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn collision_system(
    mut position_component_vector: &mut RefMut<Vec<Option<Position>>>, 
    mut rigid_body_component_vector: &mut RefMut<Vec<Option<RigidBody>>>, 
    mut collision_list_component_vector: &mut RefMut<Vec<Option<CollisionList>>>, 
    time_passed: u128
) {
    // Clear collision lists from before
    {
        let iter = collision_list_component_vector.iter_mut().filter_map(|collision_list| Some(collision_list.as_mut()?));
        for collision_list in iter {
            println!("Cleared {:?} collisions", collision_list.list.len());
            collision_list.list.clear();
        }
    }

    // Calculate new collisions
    {
        
        let iter_a = {
            let rigid_body_iter_a = rigid_body_component_vector.iter();
            let position_iter_a = position_component_vector.iter();
            rigid_body_iter_a.enumerate().zip(position_iter_a).filter_map(|((i, rigid_body), position)| Some((rigid_body.as_ref()?, position.as_ref()?, i)))
        };
        for (rigid_body_a, position_a, i_a) in iter_a {
            // Check for collision
            let min_x_a = position_a.x;
            let min_y_a = position_a.y;
            let max_x_a = position_a.x + rigid_body_a.width;
            let max_y_a = position_a.y + rigid_body_a.height;
            
            // Save collision to collision list of the object - only really do the test if the object has a collision list (wants to react to collisions)
            let mut collision_list = collision_list_component_vector.get_mut(i_a).unwrap();
            if let Some(collision_list) = collision_list {
                // collision_list.list.push(0);
                let iter_b = {
                    let rigid_body_iter_b = rigid_body_component_vector.iter();
                    let position_iter_b = position_component_vector.iter();
                    rigid_body_iter_b.enumerate().zip(position_iter_b).filter_map(|((i, rigid_body), position)| Some((rigid_body.as_ref()?, position.as_ref()?, i)))
                };
                for (rigid_body_b, position_b, i_b) in iter_b {
                    if !std::ptr::eq(rigid_body_a, rigid_body_b) {
                        let min_x_b = position_b.x;
                        let min_y_b = position_b.y;
                        let max_x_b = position_b.x + rigid_body_b.width;
                        let max_y_b = position_b.y + rigid_body_b.height;
                        if min_x_a < max_x_b && max_x_a > min_x_b && min_y_a < max_y_b && max_y_a > min_y_b {
                            // Collision hapnt - figure out which _a's side is colliding with _b?
                            println!("Collides: {:?} with {:?}", i_a, i_b);
                        }
                    }
                }
            }
        }
    }
}