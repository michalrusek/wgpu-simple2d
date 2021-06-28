pub struct Name<'a> {
    pub name: &'a str
}

pub struct Sprite {
    pub texture_id: usize,
    pub render: bool,
    pub width_normalized: f32,
    pub height_normalized: f32,
    pub z: u32 // TODO: Move Z elsewhere, proly some other component
}

pub struct Health {
    pub health: u32
}

pub struct Animation<'a> {
    pub animation_name: &'a str,
    pub running: bool,
    pub sprites: Vec<Sprite>,
    pub time_per_frame_ms: u32,
    pub time_since_last_frame: u32,
    pub current_frame_index: usize,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Gravity {
    pub affected_by_gravity: bool
}

pub struct Velocity {
    pub vel_x: f32,
    pub vel_y: f32,
}

pub struct RigidBody {
    pub width: f32,
    pub height: f32,
}

pub struct CollisionList {
    pub list: Vec<usize>
}