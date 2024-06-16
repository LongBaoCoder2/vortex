use crate::{point::Point2d, traits::*};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Player {
    pub speed: f64,
    pub health: u8,
    pub position: Point2d<f64>,
    pub direction: Point2d<f64>,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
    pub fn take_damage(&mut self, damage: u8) {
        self.health -= damage;
    }
    pub fn health(&self) -> u8 {
        self.health
    }
    pub fn speed(&self) -> f64 {
        self.speed
    }
}

impl Position<f64> for Player {
    fn position(&self) -> Point2d<f64> {
        Point2d::new(self.position.x, self.position.y)
    }

    fn set_position(&mut self, position: Point2d<f64>) {
        self.position = position;
    }
}

impl Movable<f64> for Player {
    fn accelerate(&mut self) {
        self.speed += Player::ACCELEBRATE;
    }

    fn decelerate(&mut self) {
        self.speed -= Player::ACCELEBRATE;
    }

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

    // Stop Here
    fn turn_left(&mut self) {
        self.position.x
    }

    fn turn_right(&mut self) {}
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "↑")
    }
}
