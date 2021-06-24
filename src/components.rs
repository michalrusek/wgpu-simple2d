pub struct Name<'a> {
    pub name: &'a str
}

pub struct Sprite {
    pub texture_id: usize,
    pub render: bool,
    pub p1: (f32, f32),
    pub p2: (f32, f32),
}