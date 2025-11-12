pub mod generation;
mod units;
mod player;
pub mod components;
pub mod renderables;

use cgmath::{ Vector3, Vector4, Point3, Quaternion };
use legion::{self, Schedule, IntoQuery};
use std::time::Instant;


use renderables::Renderables;
use generation::worldblocks::WorldBlocks;
use player::{ Camera };
use crate::{application::Input};
use crate::graphics::text_render::{ text_style::TextStyle, sentence::Sentence };
use components::{ time::Time, spatial::{ Direction, Position }};

pub struct Game {
    blocks: WorldBlocks,
    world: legion::World,
    pre_collision_schedule: legion::Schedule,
    post_collision_schedule: legion::Schedule,
    resources: legion::Resources,

    last_tick: Instant,
    dt: f32,
}


impl Game {
    pub fn new() -> Self {
        let blocks = WorldBlocks::test_layout();
        let mut world = legion::World::default();
        player::generate_main_player(&mut world);

        Self {
            blocks,
            world,
            pre_collision_schedule: Game::generate_precollision_schedule(),
            post_collision_schedule: Game::generate_postcollision_schedule(),
            resources: legion::Resources::default(),
            last_tick: Instant::now(),
            dt: 0.
        }
    }

    pub fn generate_precollision_schedule() -> Schedule {
        let mut scheduler = legion::Schedule::builder();

        components::input::schedule(&mut scheduler);

        return scheduler.build()
    }

    pub fn generate_postcollision_schedule() -> Schedule {
        let mut scheduler = legion::Schedule::builder();

        components::spatial::schedule(&mut scheduler);

        return scheduler.build()
    }

    pub fn tick(&mut self, input: Input) {
        // Update time
        self.dt = (Instant::now() - self.last_tick).as_secs_f32();
        self.last_tick = Instant::now();

        // Prepare resources
        self.resources.insert(Time { dt: self.dt });
        self.resources.insert(input);

        self.pre_collision_schedule.execute(&mut self.world, &mut self.resources);

        self.post_collision_schedule.execute(&mut self.world, &mut self.resources);

        components::collision::block_collide(&mut self.world, &self.blocks);
    }

    pub fn get_renderables(&mut self) -> Renderables {
        // Get object with camera
        let (cam_pos, cam_dir) = self.get_camera();

        let mut sentences = Vec::new();

        sentences.push(Sentence {
            data: "Big Dick Forever".to_owned(),
            position: Vector3::new(11., 5., 10.),
            direction: Quaternion::new(1., 0., 0., 0.),
            text_style: TextStyle {
                font: "Arial".to_owned(),
                color: Vector4::new(0.5, 1., 1., 1.),
                scale: 2.,
                affected_by_camera: true
            }
        });
        
        Renderables {
            cam_dir,
            cam_pos,
            cubes: self.blocks.get_renderable_blocks(cam_pos),
            sentences,
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