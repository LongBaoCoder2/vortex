use crate::{point::Point2d, traits::*};

use super::Player;

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Enemy {
    pub speed: f64,
    pub position: Point2d<f64>,
    pub direction: Point2d<f64>,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self {
            speed,
            position: Point2d::default(),
            direction: Point2d::default(),
        }
    }

    pub fn direction_to_player(&mut self, player: &Player) {
        let direction = &player.position - &self.position;
        self.direction = direction.normalize();
    }
}

impl Position<f64> for Enemy {
    fn position(&self) -> Point2d<f64> {
        Point2d::new(self.position.x, self.position.y)
    }

    fn set_position(&mut self, position: Point2d<f64>) {
        self.position = position;
    }
}

impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "╦")
    }
}

impl Movable<f64> for Enemy {
    fn move_forward(&mut self) {
        self.position.x += self.direction.x * self.speed;
        self.position.y += self.direction.y * self.speed;
    }

    fn forward_position(&self) -> Point2d<f64> {
        Point2d::new(
            self.position.x + self.direction.x * self.speed,
            self.position.y + self.direction.y * self.speed,
        )
    }
}
