use crate::{unit::Collectible, unit::Enemy, unit::Player, unit::PlayerBuilder, unit::Wall};

#[allow(unused)]
pub struct Game {
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
}

impl Game {
    #[allow(cippy::new_without_default)]
    pub fn new() -> Self {
        Game::builder().build()
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }

    // Test mod
    pub fn init(&mut self) {
        println!("Game initialized");
        println!("Player speed: {}", self.player.speed);
        println!("Player health: {}", self.player.health);
        println!("Player is alive: {}", self.player.is_alive());
        self.player.take_damage(self.player.health);
        println!("Player is alive: {}", self.player.is_alive());
    }

    pub fn run(&mut self) {
        self.init();
    }
}

#[derive(Debug, Default)]
pub struct GameBuilder {
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    player: PlayerBuilder,
}

impl GameBuilder {
    #[allow(cippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
            walls: Vec::new(),
            player: PlayerBuilder::new(),
        }
    }

    pub fn build(self) -> Game {
        Game {
            enemies: self.enemies,
            walls: self.walls,
            collectible: Collectible::default(),
            player: self.player.build(),
        }
    }

    pub fn walls(mut self, walls: Vec<Wall>) -> Self {
        self.walls = walls;
        self
    }

    pub fn enemies(mut self, enemies: Vec<Enemy>) -> Self {
        self.enemies = enemies;
        self
    }

    pub fn player_starting_speed(mut self, speed: f32) -> Self {
        self.player = self.player.speed(speed);
        self
    }

    pub fn player_starting_health(mut self, health: u32) -> Self {
        self.player = self.player.health(health);
        self
    }
}
