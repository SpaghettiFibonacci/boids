#[derive(Debug, Clone, Copy)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Bird {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Bird { x, y, vx, vy }
    }
}
