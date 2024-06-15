#[derive(Debug, Default)]
#[allow(unused)]
pub struct Player {
    pub speed: f32,
    pub health: u32,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
    pub fn take_damage(&mut self, damage: u32) {
        self.health -= damage;
    }
    pub fn health(&self) -> u32 {
        self.health
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
}
