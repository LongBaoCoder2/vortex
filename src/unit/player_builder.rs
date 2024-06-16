use crate::{point::Point2d, unit::Player};

#[derive(Debug, Default)]
pub struct PlayerBuilder {
    health: u8,
    speed: f64,
    position: Point2d<f64>,
    direction: Point2d<f64>,
}

impl PlayerBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn position(mut self, position: Point2d<f64>) -> Self {
        self.position = position;
        self
    }

    pub fn direction(mut self, direction: Point2d<f64>) -> Self {
        self.direction = direction;
        self
    }

    pub fn build(self) -> Player {
        Player {
            health: self.health,
            speed: self.speed,
            position: self.position,
            direction: self.direction,
            accelerate: 0.0,
        }
    }
}
