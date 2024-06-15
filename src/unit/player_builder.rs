use super::player::Player;

#[derive(Debug, Default)]
pub struct PlayerBuilder {
    health: u32,
    speed: f32,
}

impl PlayerBuilder {
    #[allow(cippy::new_without_default)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn health(mut self, health: u32) -> Self {
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
