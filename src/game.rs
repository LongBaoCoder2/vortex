use rand::rngs::ThreadRng;

use crate::{
    traits::Position,
    unit::{Collectible, Enemy, Player, PlayerBuilder, Wall},
};

#[allow(unused)]
pub struct Game {
    height: u16,
    width: u16,
    n_random_walls: u16,
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
    rng: ThreadRng,
}

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Game::builder().build()
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }

    // Test mod
    pub fn init(&mut self) {
        for i in 0..self.width {
            self.walls.push(Wall::new(i, 0));
            self.walls.push(Wall::new(i, self.height - 1));
        }

        for i in 0..self.height {
            self.walls.push(Wall::new(0, i));
            self.walls.push(Wall::new(0, self.width - 1));
        }

        for _ in 0..self.n_random_walls {
            let mut wall = Wall::default();
            wall.set_rand_position(&mut self.rng, 0..self.width - 1, 0..self.height - 1);
            self.walls.push(wall);
        }

        self.enemies.iter_mut().for_each(|enemy| {
            enemy.set_rand_position(
                &mut self.rng,
                1.0..(self.width - 1).into(),
                1.0..(self.height - 1).into(),
            )
        });

        while self
            .walls
            .iter()
            .any(|wall| wall.position == self.collectible.position)
        {
            self.collectible
                .set_rand_position(&mut self.rng, 1..self.width - 1, 1..self.height - 1)
        }
    }

    pub fn run(&mut self) {
        self.init();
        // print positions
        println!(
            "Player: x = {}, y = {}",
            self.player.position().x,
            self.player.position().y
        );
        println!(
            "Collectible: x = {}, y = {}",
            self.collectible.position().x,
            self.collectible.position().y
        );
        for enemy in &self.enemies {
            println!(
                "Enemy: x = {}, y = {}",
                enemy.position().x,
                enemy.position().y
            );
        }
        for wall in &self.walls {
            println!("Wall: x = {}, y = {}", wall.position().x, wall.position().y);
        }
    }
}

#[derive(Debug, Default)]
pub struct GameBuilder {
    height: u16,
    width: u16,
    n_random_walls: u16,
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    player_builder: PlayerBuilder,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            height: 80,
            width: 64,
            enemies: Vec::new(),
            walls: Vec::new(),
            player_builder: PlayerBuilder::new(),
            n_random_walls: 10,
        }
    }
    pub fn build(self) -> Game {
        Game {
            height: self.height,
            width: self.width,
            n_random_walls: self.n_random_walls,
            enemies: self.enemies,
            walls: self.walls,
            collectible: Collectible::default(),
            player: self.player_builder.build(),
            rng: rand::thread_rng(),
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

    pub fn player_starting_speed(mut self, speed: f64) -> Self {
        self.player_builder = self.player_builder.speed(speed);
        self
    }

    pub fn player_starting_health(mut self, health: u8) -> Self {
        self.player_builder = self.player_builder.health(health);
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn n_random_walls(mut self, n_random_walls: u16) -> Self {
        self.n_random_walls = n_random_walls;
        self
    }
}
