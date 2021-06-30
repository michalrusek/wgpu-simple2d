use std::cell::{RefCell, RefMut};
use crate::components::*;

const MAX_DOWNWARD_VELOCITY: f32 = 1.;
const VELOCITY_GAIN_PER_MS: f32 = 5. / 1000.;

pub fn gravity_system(
    mut gravity_component_vector: &mut RefMut<Vec<Option<Gravity>>>, 
    mut velocity_component_vector: &mut RefMut<Vec<Option<Velocity>>>,
    time_passed: u128
) {
    let gravity_iter = gravity_component_vector.iter_mut();
    let velocity_iter = velocity_component_vector.iter_mut();
    let zip = gravity_iter.zip(velocity_iter);
    let iter = zip.filter_map(|(gravity, velocity)| Some((gravity.as_mut()?, velocity.as_mut()?)));
    for (gravity, velocity) in iter {
        if gravity.affected_by_gravity {
            velocity.vel_y += VELOCITY_GAIN_PER_MS * time_passed as f32;
            if velocity.vel_y > MAX_DOWNWARD_VELOCITY {
                velocity.vel_y = MAX_DOWNWARD_VELOCITY;
            }
        }
    }
}