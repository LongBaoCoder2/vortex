#[derive(Debug, Default)]
#[allow(unused)]
pub struct Enemy {
    pub speed: f64,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self { speed }
    }
}
