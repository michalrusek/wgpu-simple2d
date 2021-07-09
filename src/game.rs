use crate::renderer::*;
use crate::components::*;
use crate::systems::health::*;
use crate::systems::player_movement::*;
use crate::systems::animation::*;
use crate::systems::gravity::*;
use crate::systems::physics::*;
use crate::systems::collision::*;
use crate::systems::player_animation::*;
use crate::systems::player_pineapple::*;
use crate::systems::points_ticking_down::*;
use crate::systems::flag_reached::*;
use std::any;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

enum Scenes {
    Ingame,
    GameOver,
    YouWon,
}

pub struct Game {
    target_resolution: [u32; 2],
    keyboard_input_queue: Vec<winit::event::KeyboardInput>,
    player_index: usize,
    entity_count: usize,
    component_vectors: Vec<Box<dyn ComponentsVector>>, // Vector containing other vectors - each vector here is of a component type and has components of that type;
    current_scene: Scenes
}

impl Game {
    pub fn new(target_resolution: [u32; 2]) -> Self {

        Self {target_resolution, entity_count: 0, component_vectors: Vec::new(), player_index: 0, keyboard_input_queue: Vec::new(), current_scene: Scenes::Ingame}
    }

    fn clear_scene(&mut self) {
        self.component_vectors = Vec::new();
        self.entity_count = 0;
        self.player_index = 0;
        self.keyboard_input_queue = Vec::new();
    }

    fn init_scene_in_game(&mut self, renderer: &mut Renderer) {
        // Load player
        {   
            // Add a new entity for the player
            self.player_index = self.add_entity();

            // Prepare all animations for the player
            let anim_map = {
                let mut anim_map = AnimationMap {map: HashMap::new(), current_animation_name: "", horiz_mirror: false};

                // Idle animation
                {
                    let player_texture = renderer.register_texture("res/sillyboi.png");
                    let player_texture_2 = renderer.register_texture("res/sillyboi2.png");
                    let anim_idle = Animation {
                        animation_name: "idle", 
                        time_per_frame_ms: 250,
                        time_since_last_frame: 0,
                        current_frame_index: 1,
                        running: true,
                        sprites: vec![
                            Sprite {
                                texture_id: player_texture,
                                render: true,
                                width_normalized: 64. / self.target_resolution[0] as f32,
                                height_normalized: 64. / self.target_resolution[1] as f32,
                                z: 10,
                            },
                            Sprite {
                                texture_id: player_texture_2,
                                render: true,
                                width_normalized: 64. / self.target_resolution[0] as f32,
                                height_normalized: 64. / self.target_resolution[1] as f32,
                                z: 10,
                            },
                        ]
                    };
                    anim_map.current_animation_name = anim_idle.animation_name;
                    anim_map.map.insert(anim_idle.animation_name, anim_idle);
                }

                // Running right animation
                {
                    let mut sprites: Vec<Sprite> = Vec::new();
                    for i in 0..12 {
                        let filename_prefix: String = "res/sillyboi_running_right/row-1-col-".to_owned();
                        let extension = ".png";
                        let full_filename = filename_prefix + &(i + 1).to_string() + extension;
                        let texture = renderer.register_texture(&full_filename);
                        sprites.push(Sprite {
                            texture_id: texture,
                            render: true,
                            width_normalized: 64. / self.target_resolution[0] as f32,
                            height_normalized: 64. / self.target_resolution[1] as f32,
                            z: 10,
                        });
                    }   
                    let anim = Animation {
                        animation_name: "running_right",
                        current_frame_index: 0,
                        running: true,
                        sprites,
                        time_per_frame_ms: 50,
                        time_since_last_frame: 0,
                    };
                    anim_map.current_animation_name = anim.animation_name;
                    anim_map.map.insert(anim.animation_name, anim);
                }
                
                // Running left animation
                {
                    let mut sprites: Vec<Sprite> = Vec::new();
                    for i in (0..12).rev() {
                        let filename_prefix: String = "res/sillyboi_running_left/row-1-col-".to_owned();
                        let extension = ".png";
                        let full_filename = filename_prefix + &(i + 1).to_string() + extension;
                        let texture = renderer.register_texture(&full_filename);
                        sprites.push(Sprite {
                            texture_id: texture,
                            render: true,
                            width_normalized: 64. / self.target_resolution[0] as f32,
                            height_normalized: 64. / self.target_resolution[1] as f32,
                            z: 10,
                        });
                    }   
                    let anim = Animation {
                        animation_name: "running_left",
                        current_frame_index: 0,
                        running: true,
                        sprites,
                        time_per_frame_ms: 50,
                        time_since_last_frame: 0,
                    };
                    anim_map.current_animation_name = anim.animation_name;
                    anim_map.map.insert(anim.animation_name, anim);
                }

                // Jump animation
                {
                    let mut sprites: Vec<Sprite> = Vec::new();
                    let texture = renderer.register_texture("res/sillyboi_jump/Jump (32x32).png");
                    sprites.push(Sprite {
                        texture_id: texture,
                        render: true,
                        width_normalized: 64. / self.target_resolution[0] as f32,
                        height_normalized: 64. / self.target_resolution[1] as f32,
                        z: 10,
                    });
                    let anim = Animation {
                        animation_name: "jump",
                        current_frame_index: 0,
                        running: true,
                        sprites,
                        time_per_frame_ms: 50,
                        time_since_last_frame: 0,
                    };
                    anim_map.current_animation_name = anim.animation_name;
                    anim_map.map.insert(anim.animation_name, anim);
                }

                // Fall animation
                {
                    let mut sprites: Vec<Sprite> = Vec::new();
                    let texture = renderer.register_texture("res/sillyboi_fall/Fall (32x32).png");
                    sprites.push(Sprite {
                        texture_id: texture,
                        render: true,
                        width_normalized: 64. / self.target_resolution[0] as f32,
                        height_normalized: 64. / self.target_resolution[1] as f32,
                        z: 10,
                    });
                    let anim = Animation {
                        animation_name: "fall",
                        current_frame_index: 0,
                        running: true,
                        sprites,
                        time_per_frame_ms: 50,
                        time_since_last_frame: 0,
                    };
                    anim_map.current_animation_name = anim.animation_name;
                    anim_map.map.insert(anim.animation_name, anim);
                }
                anim_map
            };
            self.add_component_to_entity(self.player_index, anim_map);
            
            // Prepare the rest of simpler components
            self.add_component_to_entity(self.player_index, Name {name: "silly boi"});
            self.add_component_to_entity(self.player_index, Health {health: 100});
            self.add_component_to_entity(self.player_index, Position {x: 100. / self.target_resolution[0] as f32, y: 0. / self.target_resolution[1] as f32});
            self.add_component_to_entity(self.player_index, Gravity {affected_by_gravity: true});
            self.add_component_to_entity(self.player_index, Velocity {vel_x: 0., vel_y: 0.});
            self.add_component_to_entity(self.player_index, RigidBody {width: 64. / self.target_resolution[0] as f32, height: 64. / self.target_resolution[1] as f32});
            self.add_component_to_entity(self.player_index, CollisionList {list: Vec::new()});
            self.add_component_to_entity(self.player_index, PlayerState{state: PlayerStateKind::Idle});
            self.add_component_to_entity(self.player_index, EntityType::Player);
            self.add_component_to_entity(self.player_index, Points{points: 10, time_since_last_point_change_ms: 0});
        }

        // Load terrain
        {   
            let terrain_texture_index = renderer.register_texture("res/platformthing.png");
            let max_squares = self.target_resolution[0] / 96 + 1;
            for square_n in 0..max_squares as usize {
                let offset: f32 = 96. / self.target_resolution[0] as f32 * square_n as f32;
                let terrain_index = self.add_entity();
                self.add_component_to_entity(terrain_index, Sprite {
                    texture_id: terrain_texture_index,
                    render: true,
                    width_normalized: 96. / self.target_resolution[0] as f32,
                    height_normalized: 96. / self.target_resolution[1] as f32,
                    z: 1,
                });
                self.add_component_to_entity(terrain_index, Position {x: offset, y: 600. / self.target_resolution[1] as f32});
                self.add_component_to_entity(terrain_index, RigidBody {width: 96. / self.target_resolution[0] as f32, height: 96. / self.target_resolution[1] as f32});
                self.add_component_to_entity(terrain_index, BlocksMovement {blocks: true});
                self.add_component_to_entity(terrain_index, EntityType::Static);
            }

            {
                let terrain_index = self.add_entity();
                self.add_component_to_entity(terrain_index, Sprite {
                    texture_id: terrain_texture_index,
                    render: true,
                    width_normalized: 96. / self.target_resolution[0] as f32,
                    height_normalized: 96. / self.target_resolution[1] as f32,
                    z: 1,
                });
                self.add_component_to_entity(terrain_index, Position {x: (self.target_resolution[0] - 96) as f32 / self.target_resolution[0] as f32, y: 504. / self.target_resolution[1] as f32});
                self.add_component_to_entity(terrain_index, RigidBody {width: 96. / self.target_resolution[0] as f32, height: 96. / self.target_resolution[1] as f32});
                self.add_component_to_entity(terrain_index, BlocksMovement {blocks: true});
                self.add_component_to_entity(terrain_index, EntityType::Static);
            }
            {
                let terrain_index = self.add_entity();
                self.add_component_to_entity(terrain_index, Sprite {
                    texture_id: terrain_texture_index,
                    render: true,
                    width_normalized: 96. / self.target_resolution[0] as f32,
                    height_normalized: 96. / self.target_resolution[1] as f32,
                    z: 1,
                });
                self.add_component_to_entity(terrain_index, Position {x: 0. / self.target_resolution[0] as f32, y: 504. / self.target_resolution[1] as f32});
                self.add_component_to_entity(terrain_index, RigidBody {width: 96. / self.target_resolution[0] as f32, height: 96. / self.target_resolution[1] as f32});
                self.add_component_to_entity(terrain_index, BlocksMovement {blocks: true});
                self.add_component_to_entity(terrain_index, EntityType::Static);
            }
            {   
                for i in 0..20 {
                    let pineapple_index = self.add_entity();
                    let mut sprites: Vec<Sprite> = Vec::new();
                    for i in 0..17 {
                        let filename_prefix: String = "res/pineapple/row-1-col-".to_owned();
                        let extension = ".png";
                        let full_filename = filename_prefix + &(i + 1).to_string() + extension;
                        let texture = renderer.register_texture(&full_filename);
                        sprites.push(Sprite {
                            texture_id: texture,
                            render: true,
                            width_normalized: 64. / self.target_resolution[0] as f32,
                            height_normalized: 64. / self.target_resolution[1] as f32,
                            z: 1,
                        });
                    }  
                    self.add_component_to_entity(pineapple_index, Animation {
                        animation_name: "idle",
                        current_frame_index: 0,
                        running: true,
                        time_per_frame_ms: 50,
                        time_since_last_frame: 0,
                        sprites
                    });
                    let pineapple_x = 200 + i * 50;
                    self.add_component_to_entity(pineapple_index, Position {x: pineapple_x as f32 / self.target_resolution[0] as f32, y: 504. / self.target_resolution[1] as f32});
                    self.add_component_to_entity(pineapple_index, RigidBody {width: 96. / self.target_resolution[0] as f32, height: 96. / self.target_resolution[1] as f32});
                    self.add_component_to_entity(pineapple_index, MarkedForDeletion {marked: false});
                    self.add_component_to_entity(pineapple_index, EntityType::Pineapple);
                }
            }

            // End flag
            {
                let flag_id = self.add_entity();
                self.add_component_to_entity(flag_id, Position {x: (self.target_resolution[0] - 96) as f32 / self.target_resolution[0] as f32, y: 376. / self.target_resolution[1] as f32});
                self.add_component_to_entity(flag_id, RigidBody {width: 128. / self.target_resolution[0] as f32, height: 128. / self.target_resolution[1] as f32});
                self.add_component_to_entity(flag_id, EntityType::EndFlag);

                // Flag animation
                let anim = {
                    let mut sprites: Vec<Sprite> = Vec::new();
                    for i in 0..10 {
                        let filename_prefix: String = "res/end_flag/row-1-col-".to_owned();
                        let extension = ".png";
                        let full_filename = filename_prefix + &(i + 1).to_string() + extension;
                        let texture = renderer.register_texture(&full_filename);
                        sprites.push(Sprite {
                            texture_id: texture,
                            render: true,
                            width_normalized: 128. / self.target_resolution[0] as f32,
                            height_normalized: 128. / self.target_resolution[1] as f32,
                            z: 0,
                        });
                    }   
                    Animation {
                        animation_name: "running_right",
                        current_frame_index: 0,
                        running: true,
                        sprites,
                        time_per_frame_ms: 50,
                        time_since_last_frame: 0,
                    }
                };

                self.add_component_to_entity(flag_id, anim);
            }
        }
    }
    fn init_scene_game_over(&mut self, renderer: &mut Renderer) {}
    fn init_scene_you_won(&mut self, renderer: &mut Renderer) {}

    pub fn init(&mut self, renderer: &mut Renderer) {
        // Initialize components and stuff here
        self.swap_scene(Scenes::Ingame, renderer);
    }

    fn swap_scene(&mut self, scene: Scenes, renderer: &mut Renderer) {
        self.clear_scene();

        match scene {
            Scenes::Ingame => { self.init_scene_in_game(renderer); },
            Scenes::GameOver => { self.init_scene_game_over(renderer); },
            Scenes::YouWon => { self.init_scene_you_won(renderer); }
        }
        self.current_scene = scene;
    }

    pub fn update(&mut self, time_passed: u128, renderer: &mut Renderer) -> bool {
        // LEFT AS AN EXAMPLE HERE ON HOW TO ITERATE AND SHIT
        // if false {
        //     let mut names = self.borrow_component_vector_mut::<Name>().unwrap();
        //     let iter = names.iter_mut().filter_map(|name| Some(name.as_mut()?));
        //     for name in iter {
        //         println!("{:?}", name.name);
        //     }
        // }
        
        let mut scene_swap_opt: Option<Scenes> = None;
        
        // Health system
        {
            if let Some(mut health_components) = self.borrow_component_vector_mut::<Health>() {
                health_system(&mut health_components);
            }
        }

        // Player movement system
        {
            if let Some(mut velocity_components) = self.borrow_component_vector_mut::<Velocity>() {
                player_movement_system(&mut velocity_components, self.player_index, &self.keyboard_input_queue);
            }
        }

        // Animation system
        {
            let mut animation_components = self.borrow_component_vector_mut::<Animation>();
            let mut animation_map_components = self.borrow_component_vector_mut::<AnimationMap>();
            animation_system(&mut animation_components, &mut animation_map_components, time_passed);
        }

        // Gravity system
        {
            if let (
                Some(mut gravity_components),
                Some(mut velocity_components)
            ) = (
                 self.borrow_component_vector_mut::<Gravity>(),
                 self.borrow_component_vector_mut::<Velocity>()
            ) {
                gravity_system(&mut gravity_components, &mut velocity_components, time_passed);
            }
        }

        // Movement system
        {
            if let (
                Some(mut velocity_components), 
                Some(mut position_components),
                Some(mut rigid_body_components),
                Some(mut blocks_movement),
            ) = (
                self.borrow_component_vector_mut::<Velocity>(), 
                self.borrow_component_vector_mut::<Position>(),
                self.borrow_component_vector_mut::<RigidBody>(),
                self.borrow_component_vector_mut::<BlocksMovement>(),
            ) {
                physics_system(&mut velocity_components, &mut position_components, &mut rigid_body_components, &mut blocks_movement, time_passed);
            }
        }

        // Collision system
        {
            if let (
                Some(mut position_components),
                Some(mut rigid_body_components),
                Some(mut collision_list_components),
            ) = (
                self.borrow_component_vector_mut::<Position>(),
                self.borrow_component_vector_mut::<RigidBody>(),
                self.borrow_component_vector_mut::<CollisionList>(),
            ) {
                collision_system(
                    &mut position_components, 
                    &mut rigid_body_components,
                    &mut collision_list_components,
                    time_passed,
                );
            }
        }

        // Player animation system
        {
            if let (
                Some(mut velocity_components),
                Some(mut animation_map_components),
            ) = (
                self.borrow_component_vector_mut::<Velocity>(), 
                self.borrow_component_vector_mut::<AnimationMap>(), 
            ) {
                player_animation_system(&mut velocity_components, &mut animation_map_components, self.player_index);
            }
        }

        // Player pineapple system
        {
            if let (
                Some(mut collision_list_components),
                Some(mut marked_for_deletion_components),
                Some(mut entity_type_components),
                Some(mut points_components),
            ) = (
                self.borrow_component_vector_mut::<CollisionList>(),
                self.borrow_component_vector_mut::<MarkedForDeletion>(),
                self.borrow_component_vector_mut::<EntityType>(),
                self.borrow_component_vector_mut::<Points>(),
            ) {
                player_pineapple_system(&collision_list_components, &mut marked_for_deletion_components, &entity_type_components, &mut points_components, self.player_index);
            }
        }

        // Flag reached system
        if let (
            Some(collision_list_components),
            Some(entity_type_components),
        ) = (
            self.borrow_component_vector_mut::<CollisionList>(),
            self.borrow_component_vector_mut::<EntityType>(),
        ) {
            if flag_reached_system(&collision_list_components, &entity_type_components, self.player_index) {
                scene_swap_opt = Some(Scenes::YouWon);
            }
        }
        
        // Points ticking down system
        if let (
            Some(mut points_components),
        ) = (
            self.borrow_component_vector_mut::<Points>(),
        ) {
            if points_ticking_down(&mut points_components, time_passed, self.player_index) {
                scene_swap_opt = Some(Scenes::GameOver);
            }
        }

        // Clear up the keyboard input queue
        {
            self.keyboard_input_queue.clear();
        }

        // Special system that removes unused entities
        {
            let mut entities_for_deletion: Vec<usize> = Vec::new();
            if let Some(marked_for_deletion_components) = self.borrow_component_vector_mut::<MarkedForDeletion>() {
                let iter = marked_for_deletion_components.iter().enumerate();
                for (marked_for_deletion, index) in iter.filter_map(|(index, marked_for_deletion)| Some((marked_for_deletion.as_ref()?, index))) {
                    if marked_for_deletion.marked {
                        entities_for_deletion.push(index);
                    }
                }
            }
            for index_to_delete in entities_for_deletion {
                self.delete_entity(index_to_delete);
            }
        }

        if let Some(scene_to_swap) = scene_swap_opt {
            self.swap_scene(scene_to_swap, renderer);
        }

        false
    }

    pub fn process_keyboard_input(&mut self, input: &winit::event::KeyboardInput) {
        // Save inputs probably - the inputs can come more than once during one frame and so it'd be good to defer handling them until update() is run
        self.keyboard_input_queue.push(*input);
    }

    fn get_world_renderables(&self) -> Vec<Renderable> {
        let mut to_return: Vec<Renderable> = Vec::new();
        let mut z_buffer: Vec<u32> = Vec::new(); // TODO: Could do Z-checks work on a GPU instead proly

        // Sprite rendering function
        let mut render_sprite = |position: &Position, sprite: &Sprite, horiz_mirror: bool| {
            if sprite.render {
                let (x1, y1) = (position.x, position.y);
                let (x2, y2) = (position.x + sprite.width_normalized, position.y + sprite.height_normalized);
                let new_renderable = Renderable{ p1: [x1, y1], p2: [x2, y2], texture_id: sprite.texture_id, use_texture_size: false, horiz_mirror };
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
        };

        // Render simple sprites
        {
            if let (Some(sprites), Some(positions)) = (self.borrow_component_vector_mut::<Sprite>(), self.borrow_component_vector_mut::<Position>()) {
                let zip = positions.iter().zip(sprites.iter());
                let iter = zip.filter_map(|(position, sprite)| Some((position.as_ref()?, sprite.as_ref()?)));
                for (position, sprite) in iter {
                    render_sprite(&position, &sprite, false);
                }
            }
        }

        // Render simple animations
        {
            if let (Some(animations), Some(positions)) = (self.borrow_component_vector_mut::<Animation>(), self.borrow_component_vector_mut::<Position>()) {
                let zip = positions.iter().zip(animations.iter());
                let iter = zip.filter_map(|(position, animation)| Some((position.as_ref()?, animation.as_ref()?)));
                for (position, animation) in iter {
                    let sprite = animation.sprites.get(animation.current_frame_index).unwrap();
                    render_sprite(&position, &sprite, false);
                }
            }
        }

        // Render animation maps
        {
            if let (Some(animation_maps), Some(positions)) = (self.borrow_component_vector_mut::<AnimationMap>(), self.borrow_component_vector_mut::<Position>()) {
                let zip = positions.iter().zip(animation_maps.iter());
                let iter = zip.filter_map(|(position, animation_map)| Some((position.as_ref()?, animation_map.as_ref()?)));
                for (position, animation_map) in iter {
                    if let Some(animation) = animation_map.map.get(animation_map.current_animation_name) {
                        if let Some(sprite) = animation.sprites.get(animation.current_frame_index) {
                            render_sprite(&position, &sprite, animation_map.horiz_mirror);
                        }
                    }   
                }
            }
        }

        to_return
    }

    fn get_ui_renderables(&self) -> (Vec<Renderable>, Vec<RenderableText>) {
        // TODO: Render ui
        let mut renderables: Vec<Renderable> = Vec::new();
        let mut z_buffer: Vec<u32> = Vec::new(); // TODO: Could do Z-checks work on a GPU instead proly

        // Sprite rendering function
        let mut render_sprite = |position: &Position, sprite: &Sprite, horiz_mirror: bool| {
            if sprite.render {
                let (x1, y1) = (position.x, position.y);
                let (x2, y2) = (position.x + sprite.width_normalized, position.y + sprite.height_normalized);
                let new_renderable = Renderable{ p1: [x1, y1], p2: [x2, y2], texture_id: sprite.texture_id, use_texture_size: false, horiz_mirror };
                if renderables.is_empty() {
                    renderables.push(new_renderable);
                    z_buffer.push(sprite.z);
                } else {
                    // find the spot for the new thing
                    let mut new_index = renderables.len();
                    for (i, _) in renderables.iter().enumerate() {
                        if z_buffer.get(i).unwrap() < &sprite.z {
                            new_index = i;
                            break;
                        }
                    }
                    renderables.insert(new_index, new_renderable);
                    z_buffer.insert(new_index, sprite.z);
                }
            }
        };
        let mut renderable_texts: Vec<RenderableText> = Vec::new();

        match self.current_scene {
            Scenes::Ingame => {
                // Points
                {
                    if let Some(points_component_vector) = self.borrow_component_vector_mut::<Points>() {
                        if let Some(Some(player_points)) = points_component_vector.get(self.player_index) {
                            let points_prefix: String = "Points: ".to_owned();
                            renderable_texts.push(RenderableText {
                                color: [1., 0., 0., 1.],
                                size: 16.,
                                text: (points_prefix + &player_points.points.to_string()),
                                x: 0.02,
                                y: 0.02,
                            });
                        }
                    }
                }
            },
            Scenes::GameOver => {
                renderable_texts.push(RenderableText {
                    color: [1., 1., 1., 1.],
                    size: 64.,
                    text: "Game Over".to_owned(),
                    x: 0.27,
                    y: 0.4,
                });
                renderable_texts.push(RenderableText {
                    color: [1., 1., 1., 1.],
                    size: 32.,
                    text: "Press (ESC) to exit.".to_owned(),
                    x: 0.25,
                    y: 0.8,
                });
            },
            Scenes::YouWon => {
                renderable_texts.push(RenderableText {
                    color: [1., 1., 1., 1.],
                    size: 64.,
                    text: "You won!".to_owned(),
                    x: 0.3,
                    y: 0.4,
                });
                renderable_texts.push(RenderableText {
                    color: [1., 1., 1., 1.],
                    size: 32.,
                    text: "Press (ESC) to exit.".to_owned(),
                    x: 0.25,
                    y: 0.8,
                });
            }
        }
        
        (renderables, renderable_texts)
    }

    pub fn get_renderables(&self) -> (Vec<Renderable>, Vec<RenderableText>) {
        let (mut ui_renderables, ui_renderable_texts) = self.get_ui_renderables();
        let mut world_renderables = self.get_world_renderables();
        ui_renderables.append(&mut world_renderables);

        
        (ui_renderables, ui_renderable_texts)
    }

    fn add_entity(&mut self) -> usize {
        // TODO: USE INDEXES OF ENTITIES THAT WERE ALREADY DELETED HERE TO SAVE MEMORY!
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

    fn delete_entity(&mut self, entity_index: usize) {
        for components_vector in self.component_vectors.iter_mut() {
            components_vector.remove_component_for_entity(entity_index);
        }
    }
}

trait ComponentsVector {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn remove_component_for_entity(&mut self, entity_index: usize);
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
    fn remove_component_for_entity(&mut self, entity_index: usize) {
        if let Some(comp) = self.get_mut().get_mut(entity_index) {
            *comp = None;
        }
    }
}