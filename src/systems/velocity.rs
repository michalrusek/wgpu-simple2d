use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn velocity_system(
    mut velocity_component_vector: &mut RefMut<Vec<Option<Velocity>>>, 
    mut position_component_vector: &mut RefMut<Vec<Option<Position>>>, 
    time_passed: u128
) {
    let velocity_iter = velocity_component_vector.iter_mut();
    let position_iter = position_component_vector.iter_mut();
    let zip = velocity_iter.zip(position_iter);
    let iter = zip.filter_map(|(velocity, position)| Some((velocity.as_mut()?, position.as_mut()?)));
    for (velocity, position) in iter {
        position.x += velocity.vel_x * time_passed as f32 / 1000.;
        position.y += velocity.vel_y * time_passed as f32 / 1000.;
    }
}