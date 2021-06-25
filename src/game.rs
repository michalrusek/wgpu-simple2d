use crate::renderer::*;
use crate::components::*;
use crate::systems::health::*;
use crate::systems::player_movement::*;
use std::cell::{RefCell, RefMut};

pub struct Game {
    target_resolution: [u32; 2],
    keyboard_input_queue: Vec<winit::event::KeyboardInput>,
    player_index: usize,
    entity_count: usize,
    component_vectors: Vec<Box<dyn ComponentsVector>> // Vector containing other vectors - each vector here is of a component type and has components of that type;
}

impl Game {
    pub fn new(target_resolution: [u32; 2]) -> Self {

        Self {target_resolution, entity_count: 0, component_vectors: Vec::new(), player_index: 0, keyboard_input_queue: Vec::new()}
    }

    pub fn init(&mut self, renderer: &mut Renderer) {
        // Initialize components and stuff here

        // Load player
        {   
            let player_texture = renderer.register_texture("res/sillyboi.png");
            self.player_index = self.add_entity();
            self.add_component_to_entity(self.player_index, Sprite {
                texture_id: player_texture,
                render: true,
                p1: (100. / self.target_resolution[0] as f32, 100. / self.target_resolution[1] as f32),
                p2: (132. / self.target_resolution[0] as f32, 132. / self.target_resolution[1] as f32),
                z: 1000,
            });
            self.add_component_to_entity(self.player_index, Name {name: "silly boi"});
        }

        // Load terrain
        {   
            let terrain_texture_index = renderer.register_texture("res/platformthing.png");
            {
                let terrain_index = self.add_entity();
                self.add_component_to_entity(terrain_index, Sprite {
                    texture_id: terrain_texture_index,
                    render: true,
                    p1: (100. / self.target_resolution[0] as f32, 132. / self.target_resolution[1] as f32),
                    p2: (148. / self.target_resolution[0] as f32, 180. / self.target_resolution[1] as f32),
                    z: 0,
                });
            }
            {
                let terrain_index = self.add_entity();
                self.add_component_to_entity(terrain_index, Sprite {
                    texture_id: terrain_texture_index,
                    render: true,
                    p1: (180. / self.target_resolution[0] as f32, 132. / self.target_resolution[1] as f32),
                    p2: (228. / self.target_resolution[0] as f32, 180. / self.target_resolution[1] as f32),
                    z: 0,
                });
            }
        }
    }

    pub fn update(&mut self, time_passed: u128) {
        // LEFT AS AN EXAMPLE HERE ON HOW TO ITERATE AND SHIT
        // if false {
        //     let mut names = self.borrow_component_vector_mut::<Name>().unwrap();
        //     let iter = names.iter_mut().filter_map(|name| Some(name.as_mut()?));
        //     for name in iter {
        //         println!("{:?}", name.name);
        //     }
        // }

        // Health system
        {
            if let Some(mut health_components) = self.borrow_component_vector_mut::<Health>() {
                health_system(&mut health_components);
            }
        }

        // Player movement system
        {
            if let Some(mut sprite_components) = self.borrow_component_vector_mut::<Sprite>() {
                player_movement_system(&mut sprite_components, self.player_index, &self.keyboard_input_queue);
            }
        }

        // Clear up the keyboard input queue
        {
            self.keyboard_input_queue.clear();
        }
    }

    pub fn process_keyboard_input(&mut self, input: &winit::event::KeyboardInput) {
        // Save inputs probably - the inputs can come more than once during one frame and so it'd be good to defer handling them until update() is run
        self.keyboard_input_queue.push(*input);
    }

    pub fn get_renderables(&self) -> Vec<Renderable> {
        let mut to_return: Vec<Renderable> = Vec::new();
        let mut z_buffer: Vec<u32> = Vec::new(); // TODO: Could do Z-checks work on a GPU instead proly

        // Get all sprites
        let sprites = self.borrow_component_vector_mut::<Sprite>().unwrap();
        let sprite_iter = sprites.iter().filter(|sprite| matches!(sprite, Some(sprite)));
        for sprite_opt in sprite_iter {
            if let Some(sprite) = sprite_opt {
                if sprite.render {
                    let (x1, y1) = sprite.p1;
                    let (x2, y2) = sprite.p2;
                    let new_renderable = Renderable{ p1: [x1, y1], p2: [x2, y2], texture_id: sprite.texture_id, use_texture_size: false };
                    if to_return.is_empty() {
                        to_return.push(new_renderable);
                        z_buffer.push(sprite.z);
                    } else {
                        // find the spot for the new thing
                        let mut new_index = to_return.len();
                        for (i, _) in to_return.iter().enumerate() {
                            if z_buffer.get(i).unwrap() < &sprite.z {
                                new_index = i;
                                break;
                            }
                        }
                        to_return.insert(new_index, new_renderable);
                        z_buffer.insert(new_index, sprite.z);
                    }
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