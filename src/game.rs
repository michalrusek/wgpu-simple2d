use crate::renderer::*;
use crate::components::*;
use std::cell::{RefCell, RefMut};

pub struct Game {
    target_resolution: [u32; 2],
    entity_count: usize,
    component_vectors: Vec<Box<dyn ComponentsVector>> // Vector containing other vectors - each vector here is of a component type and has components of that type;
}

impl Game {
    pub fn new(target_resolution: [u32; 2]) -> Self {

        Self {target_resolution, entity_count: 0, component_vectors: Vec::new()}
    }

    pub fn init(&mut self, renderer: &mut Renderer) {
        // Initialize components and shit here?
        let new_id = self.add_entity();
        self.add_component_to_entity(new_id, Name { name: "sss" });
        
        let another_id = self.add_entity();
        self.add_component_to_entity(another_id, Sprite { 
            texture_id: renderer.register_texture("res/test.png"),
            render: true,
            p1: (100. / self.target_resolution[0] as f32, 100. / self.target_resolution[1] as f32)
        });
    }

    pub fn update(&mut self, time_passed: u128) {
        // LEFT AS AN EXAMPLE HERE ON HOW TO ITERATE AND SHIT
        if false {
            let mut names = self.borrow_component_vector_mut::<Name>().unwrap();
            let iter = names.iter_mut().filter_map(|name| Some(name.as_mut()?));
            for name in iter {
                println!("{:?}", name.name);
            }
        }
    }

    pub fn get_renderables(&self) -> Vec<Renderable> {
        let mut to_return: Vec<Renderable> = Vec::new();

        // Get all sprites
        let mut sprites = self.borrow_component_vector_mut::<Sprite>().unwrap();
        let sprite_iter = sprites.iter().filter(|sprite| matches!(sprite, Some(sprite)));
        for sprite_opt in sprite_iter {
            if let Some(sprite) = sprite_opt {
                if sprite.render {
                    to_return.push(Renderable{ p1: [sprite.p1.0, sprite.p1.1], p2: [0., 0.], texture_id: sprite.texture_id, use_texture_size: true });
                }
            }
        }

        to_return
    }

    fn add_entity(&mut self) -> usize {
        let new_id = self.entity_count;
        for components_vector in self.component_vectors.iter_mut() {
            components_vector.push_none();
        }
        self.entity_count += 1;
        new_id
    }

    fn add_component_to_entity<a: 'static>(&mut self, entity_id: usize, component: a) {
        for component_vector in self.component_vectors.iter_mut() {
            // See if there's a vector already for this component type and try adding the component there
            if let Some(component_vector) = component_vector
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<a>>>>() {
                    component_vector.borrow_mut()[entity_id] = Some(component);
                    return;
                }
        }

        // No vector for this component type yet; create one and add component to it;
        let mut new_components_vector: Vec<Option<a>> = Vec::with_capacity(self.entity_count);

        // Fill out None for all existing entities
        for _ in 0..self.entity_count {
            new_components_vector.push(None);
        }

        // Add the component to the actual entity
        new_components_vector[entity_id] = Some(component);

        // Add the vector of components for all entities to the state
        // Note to your future self: It's a Box because it has to go on the heap (as the size is unknown at compile time); 
        //  it's a RefCell so that you can borrow it proper during runtime
        self.component_vectors.push(Box::new(RefCell::new(new_components_vector)));
    }

    fn borrow_component_vector_mut<a: 'static>(&self,) -> Option<RefMut<Vec<Option<a>>>> {
        for component_vector in self.component_vectors.iter() {
            if let Some(component_vector) = component_vector
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<a>>>>() {
                    return Some(component_vector.borrow_mut());
                }
        }
        None
    }
}

trait ComponentsVector {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentsVector for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}