pub mod generation;
mod units;
mod player;
mod player2;
mod collider;
pub mod components;
pub mod renderables;

use cgmath::{ Vector3, Point3 };
use legion::{self, Schedule, Resources, IntoQuery};
use std::time::Instant;

use player2::Player;
use renderables::Renderables;
use generation::worldblocks::WorldBlocks;
use player::{ Camera, generate_main_player };
use crate::{application::Input};
use components::{ time::Time, spatial::{ Direction, Position }};

pub struct Game {
    blocks: WorldBlocks,
    world: legion::World,
    schedule: legion::Schedule,
    resources: legion::Resources,

    last_tick: Instant,
}


impl Game {
    pub fn new() -> Self {
        let blocks = WorldBlocks::test_layout();
        let mut world = legion::World::default();
        player::generate_main_player(&mut world);
        //let player = Player::new();

        Self {
            blocks,
            world,
            schedule: Game::generate_schedule(),
            resources: legion::Resources::default(),
            last_tick: Instant::now()
        }
    }

    pub fn generate_schedule() -> Schedule {
        let mut scheduler = legion::Schedule::builder();

        components::input::schedule(&mut scheduler);
        components::spatial::schedule(&mut scheduler);

        return scheduler.build()
    }

    pub fn tick(&mut self, input: Input) {
        let dt = (Instant::now() - self.last_tick).as_secs_f32();
        self.last_tick = Instant::now();

        self.resources.insert(Time { dt });
        self.resources.insert(input);

        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn get_renderables(&mut self) -> Renderables {
        // Get object with camera
        let (cam_pos, cam_dir) = self.get_camera();
        
        Renderables {
            cam_dir,
            cam_pos,
            cubes: self.blocks.get_renderable_blocks(cam_pos),
        }
    }

    fn get_camera(&mut self) -> (Point3<f32>, Vector3<f32>) {
        let mut query = <(&Camera, &Direction, &Position)>::query();

        let mut iter = query.iter_mut(&mut self.world);
        let (_cam, dir, pos) = match iter.next().ok_or("empty iterator") {
            Ok(value) => value,
            Err(_) => panic!("No entity with camera available!")
        };
        if iter.next().is_some() {
            panic!("More than one entity with a camera detected!");
        };

        return (pos.vector, dir.vector);
    }
}