use crate::{point::Point2d, traits::*};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Player {
    pub speed: f64,
    pub health: u8,
    pub position: Point2d<f64>,
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

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "↑")
    }
}
