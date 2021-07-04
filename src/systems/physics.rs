use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn physics_system(
    mut velocity_component_vector: &mut RefMut<Vec<Option<Velocity>>>, 
    mut position_component_vector: &mut RefMut<Vec<Option<Position>>>, 
    mut rigid_body_component_vector: &mut RefMut<Vec<Option<RigidBody>>>, 
    mut blocks_movement_component_vector: &mut RefMut<Vec<Option<BlocksMovement>>>, 
    time_passed: u128
) {
    // Move multiple times in smaller steps so we get as close to the other rigid_body as possible?
    let steps: usize = 4;

    for _ in 0..steps {
        let time_passed: f32 = time_passed as f32 / steps as f32;

        // Collect movement_allowed_stuff
        let movement_allowed_vector: Vec<AcceptMovement> = {
            let mut movement_allowed: Vec<AcceptMovement> = vec![AcceptMovement::Neither; position_component_vector.len()];

            let velocity_iter = velocity_component_vector.iter().enumerate();
            let position_iter = position_component_vector.iter();
            let rigid_body_iter = rigid_body_component_vector.iter();
            let zip = velocity_iter.zip(position_iter.zip(rigid_body_iter));
            let iter = zip.filter_map(|((index, velocity), (position, rigid_body))| Some((velocity.as_ref()?, position.as_ref()?, rigid_body.as_ref()?, index)));

            // Collect info if we're accepting the X or Y change (or both)
            for (velocity, position, rigid_body, index) in iter {
                
                // Try moving on X, check for collisions and record info
                let accept_x = {
                    let new_pos = Position {x: position.x + velocity.vel_x * time_passed / 1000., y: position.y};
                    !collides_with_another_rb(&new_pos, &rigid_body, &position_component_vector, &rigid_body_component_vector)
                };
                
                // Try moving on Y, check for collisions and record info
                let accept_y = {
                    let new_pos = Position {x: position.x, y: position.y + velocity.vel_y * time_passed / 1000.};
                    !collides_with_another_rb(&new_pos, &rigid_body, &position_component_vector, &rigid_body_component_vector)
                };

                let accept_both = {
                    let new_pos = Position {x: position.x + velocity.vel_x * time_passed / 1000., y: position.y + velocity.vel_y * time_passed / 1000.};
                    !collides_with_another_rb(&new_pos, &rigid_body, &position_component_vector, &rigid_body_component_vector)
                };

                let accept_movement = {
                    if accept_both {
                        AcceptMovement::Both
                    } else if accept_x {
                        AcceptMovement::OnX
                    } else if accept_y {
                        AcceptMovement::OnY
                    } else {
                        AcceptMovement::Neither
                    }
                };
                movement_allowed[index] = accept_movement;
            }

            movement_allowed
        };

        // Apply movement based on the allowed stuff
        {
            let velocity_iter = velocity_component_vector.iter_mut().enumerate();
            let position_iter = position_component_vector.iter_mut();
            let zip = velocity_iter.zip(position_iter);
            let iter = zip.filter_map(|((index, velocity), position)| Some((velocity.as_mut()?, position.as_mut()?, index)));
            for (velocity, position, index) in iter {
                if let Some(movement_allowed) = movement_allowed_vector.get(index) {
                    match movement_allowed {
                        AcceptMovement::Both => {
                            position.x += velocity.vel_x * time_passed / 1000.;
                            position.y += velocity.vel_y * time_passed / 1000.;
                        },
                        AcceptMovement::OnX => {
                            position.x += velocity.vel_x * time_passed / 1000.;
                            velocity.vel_y = 0.;
                        },
                        AcceptMovement::OnY => {
                            position.y += velocity.vel_y * time_passed / 1000.;
                            velocity.vel_x = 0.;
                        }
                        AcceptMovement::Neither => {
                            velocity.vel_x = 0.;
                            velocity.vel_y = 0.;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum AcceptMovement {
    OnX,
    OnY,
    Both,
    Neither
}

fn collides_with_another_rb (
    position_a: &Position, 
    rigid_body_a: &RigidBody, 
    all_positions: &Vec<Option<Position>>, 
    all_rigid_bodies: &Vec<Option<RigidBody>>
) -> bool {
    // Simple AABB collision detection
    let min_x_a = position_a.x;
    let min_y_a = position_a.y;
    let max_x_a = position_a.x + rigid_body_a.width;
    let max_y_a = position_a.y + rigid_body_a.height;
    let iter_b = {
        let rigid_body_iter_b = all_rigid_bodies.iter();
        let position_iter_b = all_positions.iter();
        rigid_body_iter_b.enumerate().zip(position_iter_b).filter_map(|((i, rigid_body), position)| Some((rigid_body.as_ref()?, position.as_ref()?, i)))
    };
    for (rigid_body_b, position_b, i_b) in iter_b {
        if !std::ptr::eq(rigid_body_a, rigid_body_b) {
            let min_x_b = position_b.x;
            let min_y_b = position_b.y;
            let max_x_b = position_b.x + rigid_body_b.width;
            let max_y_b = position_b.y + rigid_body_b.height;
            if min_x_a < max_x_b && max_x_a > min_x_b && min_y_a < max_y_b && max_y_a > min_y_b {
                return true
            }
        }
    }
    false
}