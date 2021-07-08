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

pub struct AnimationMap<'a> {
    pub map: std::collections::HashMap<&'a str, Animation<'a>>,
    pub horiz_mirror: bool,
    pub current_animation_name: &'a str,
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

pub struct Points {
    pub points: u32,
    pub time_since_last_point_change_ms: u32,
}

pub struct RigidBody {
    pub width: f32,
    pub height: f32,
}

pub struct CollisionList {
    pub list: Vec<Collision>
}

#[derive(Debug, PartialEq)]
pub enum CollisionSide {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM
}
pub struct Collision {
    pub collided_with: usize,
    pub side: CollisionSide,
    pub x_diff: f32,
    pub y_diff: f32,
}

pub struct BlocksMovement {
    pub blocks: bool,
}

pub struct PlayerState {
    pub state: PlayerStateKind
}

#[derive(PartialEq, Debug)]
pub enum PlayerStateKind {
    Idle,
    RunningLeft,
    RunningRight,
    Falling,
}

pub enum EntityType {
    Player,
    Static,
    Pineapple,
    EndFlag,
}

pub struct MarkedForDeletion {
    pub marked: bool,
}