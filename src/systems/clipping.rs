use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn clipping_system(
    mut position_component_vector: &mut RefMut<Vec<Option<Position>>>, 
    mut rigid_body_component_vector: &mut RefMut<Vec<Option<RigidBody>>>, 
    mut collision_list_component_vector: &mut RefMut<Vec<Option<CollisionList>>>,
) {
    return;
    let pos_iter = position_component_vector.iter_mut();
    let rigid_b_iter = rigid_body_component_vector.iter();
    let collision_list_iter = collision_list_component_vector.iter();
    let zip = pos_iter.zip(rigid_b_iter.zip(collision_list_iter));
    let iter = zip.filter_map(|(position, (rigid_body, collision_list))| { Some((position.as_mut()?, rigid_body.as_ref()?, collision_list.as_ref()?)) });
    for (position, rigid_body, collision_list) in iter {
        if !collision_list.list.is_empty() {
            // Naively move the position so that it doesn't collide with collisions
            // IT MIGHT lead to some stupid cases where e.g. the current body is stuck between two other bodies
            // god knows what'll happen then
            let mut max_diff = 0.;
            let mut biggest_collision_index: usize = 0;
            for (i, collision) in collision_list.list.iter().enumerate() {
                if (collision.side == CollisionSide::LEFT || collision.side == CollisionSide::RIGHT) && collision.x_diff > max_diff {
                    max_diff = collision.x_diff;
                    biggest_collision_index = i;
                }
                if (collision.side == CollisionSide::TOP || collision.side == CollisionSide::BOTTOM) && collision.y_diff > max_diff {
                    max_diff = collision.y_diff;
                    biggest_collision_index = i;
                }
                println!("Collision side for {:?}: {:?}", collision.collided_with, collision.side);
                
            }
            if let Some(biggest_collision) = collision_list.list.get(biggest_collision_index) {
                match biggest_collision.side {
                    CollisionSide::BOTTOM => {
                        position.y -= rigid_body.height - biggest_collision.y_diff;
                    },
                    CollisionSide::TOP => {
                        if let Some(Some(other_rigid_body)) = rigid_body_component_vector.get(biggest_collision.collided_with) {
                            position.y += other_rigid_body.height - biggest_collision.y_diff;
                        }
                    },
                    CollisionSide::LEFT => {
                        if let Some(Some(other_rigid_body)) = rigid_body_component_vector.get(biggest_collision.collided_with) {
                            position.x += other_rigid_body.width - biggest_collision.x_diff;
                        }
                    },
                    CollisionSide::RIGHT => {
                        position.x -= rigid_body.width - biggest_collision.x_diff;
                    }
                }
            }

        }
    }
}

