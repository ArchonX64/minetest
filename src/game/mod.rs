mod generation;
mod units;
mod player;
pub mod renderables;

use std::time::Instant;

use player::Player;
use renderables::Renderables;
use generation::world::World;
use crate::application::Input;

pub struct Game {
    world: World,
    player: Player,

    last_tick: Instant,
}

impl Game {
    pub fn new() -> Self {
        let world = World::test_layout();
        let player = Player::new();

        Self {
            world,
            player,
            last_tick: Instant::now()
        }
    }

    pub fn tick(&mut self, input: Input) {
        let dt = Instant::now() - self.last_tick;
        self.last_tick = Instant::now();
    }

    pub fn get_renderables(&mut self) -> Renderables {
        Renderables {
            cam_dir: self.player.direction,
            cubes: self.world.get_renderable_blocks(self.player.position),
        }
    }
}