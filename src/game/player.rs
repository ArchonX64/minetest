use cgmath::{ Vector3, Point3 };
use legion::World;

use super::components::{ spatial::*, input::*, collision::*};

pub struct Camera;

pub fn generate_main_player(world: &mut World) {
    world.push((
        Position { vector: Point3 { x: 10., y: 20., z: 10. }},
        Velocity::zero(),
        Direction::zero(),
        Gravity,
        HumanoidKeyboardMovement { speed: 0.1, jump_vel: 2. },
        MouseLook::base(10., 1.),
        Camera,
        BoxCollider { bounds: Vector3 { x: 1., y: 2., z: 1.} },
        CollidesWithBlocks,
    ));
}