#[derive(Debug, Default)]
#[allow(unused)]
pub struct Enemy {
    pub speed: f32,
}

impl Enemy {
    pub fn with_speed(speed: f32) -> Self {
        Self { speed }
    }
}
