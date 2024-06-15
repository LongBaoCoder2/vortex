#[derive(Debug, Default)]
#[allow(unused)]
pub struct Player {
    pub speed: f64,
    pub health: u8,
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
