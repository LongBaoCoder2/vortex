use crate::{point::Point2d, traits::*};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Player {
    pub speed: f64,
    pub health: u8,
    pub position: Point2d<f64>,
    pub direction: Point2d<f64>,
    pub direction_head: u8,
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
        self.speed = (self.speed + Player::ACCELEBRATE).min(Player::MAX_SPEED);
    }

    fn decelerate(&mut self) {
        self.speed = (self.speed + Player::ACCELEBRATE).max(0.0);
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

    fn turn_left(&mut self) {
        let (x, y) = (self.direction.x, self.direction.y);
        self.position.x = (x - y) * 1.0 / (2.0 as f64).sqrt();
        self.position.y = (x + y) * 1.0 / (2.0 as f64).sqrt();
        match self.direction_head {
            0 => self.direction_head = 7,
            _ => self.direction_head -= 1,
        }
    }

    fn turn_right(&mut self) {
        let (x, y) = (self.direction.x, self.direction.y);
        self.position.x = (x + y) * 1.0 / (2.0 as f64).sqrt();
        self.position.y = (x - y) * 1.0 / (2.0 as f64).sqrt();
        match self.direction_head {
            7 => self.direction_head = 0,
            _ => self.direction_head += 1,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "↑")
    }
}
