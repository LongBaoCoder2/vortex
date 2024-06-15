use super::player::Player;

#[derive(Debug, Default)]
pub struct PlayerBuilder {
    health: u8,
    speed: f64,
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

    pub fn build(self) -> Player {
        Player {
            health: self.health,
            speed: self.speed,
        }
    }
}
