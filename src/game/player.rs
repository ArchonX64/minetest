use legion::World;

use super::components::{ spatial::{ Position, Velocity, Direction, Gravity}, input::{ HumanoidKeyboardMovement, MouseLook } };

pub struct Camera;

pub fn generate_main_player(world: &mut World) {
    world.push((
        Position::zero(),
        Velocity::zero(),
        Direction::zero(),
        Gravity,
        HumanoidKeyboardMovement { speed: 0.1 },
        MouseLook::base(10., 1.),
        Camera
    ));
}