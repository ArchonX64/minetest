use crate::{application::Input, game::components::input::HumanoidKeyboardMovement};

use super::time::Time;

use cgmath::{ Vector3, Point3, Zero };
use legion::{ system, systems::Builder } ;
use winit::keyboard::KeyCode;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub vector: Point3<f32>
}

impl Position {
    pub fn zero() -> Self {
        return Self { vector: Point3{x: 0., y: 0., z: 0.} }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Velocity {
    pub vector: Vector3<f32>
}

impl Velocity {
    pub fn zero() -> Self {
        return Self { vector: Vector3::zero() }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Direction {
    pub vector: Vector3<f32>
}

impl Direction {
    pub fn zero() -> Self {
        return Self { vector: Vector3::zero() }
    }
}

pub struct Gravity;

#[system(for_each)]
fn apply_velocity(pos: &mut Position, vel: &Velocity, #[resource] time: &Time ) {
    pos.vector += vel.vector * time.dt;
}

const GRAVITY: f32 = 3.;
#[system(for_each)]
fn apply_gravity(vel: &mut Velocity, _grav: &Gravity, #[resource] time: &Time) {
    vel.vector.y -= GRAVITY * time.dt;
}

pub fn schedule(scheduler: &mut Builder) {
    scheduler.add_system(apply_gravity_system());
    scheduler.add_system(apply_velocity_system());
}