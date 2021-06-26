use std::cell::{RefCell, RefMut};
use crate::components::*;

pub fn health_system(mut health_component_vector: &mut RefMut<Vec<Option<Health>>>) {
    let iterator = health_component_vector.iter_mut().filter_map(|health| Some(health.as_mut()?));
    for health in iterator {
        if health.health > 0 {
            health.health -= 1;
            println!("new health: {:?}", health.health);
        }
    }
}