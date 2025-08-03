use cgmath::Vector3;
use legion::World;

use super::components::{ spatial::*, input::*, collision::*};

pub struct Camera;

pub fn generate_main_player(world: &mut World) {
    world.push((
        Position::zero(),
        Velocity::zero(),
        Direction::zero(),
        Gravity,
        HumanoidKeyboardMovement { speed: 0.1 },
        MouseLook::base(10., 1.),
        Camera,
        BoxCollider { bounds: Vector3 { x: 1., y: 2., z: 1.} },
        CollidesWithBlocks,
    ));
}