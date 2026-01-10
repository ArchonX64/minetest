use cgmath::{InnerSpace, Vector3, Zero};
use legion::{system, systems::Builder};
use winit::{event::MouseButton, keyboard::KeyCode};

use super::spatial::{Direction, Position};
use super::time::Time;
use crate::game::components::spatial::Velocity;
use crate::game::generation::worldblocks::WorldBlocks;
use crate::{application::Input, util::lerp};

pub struct PlayerInput {
    pub speed: f32,
    pub jump_vel: f32,
    pub reach: f32,
}

pub struct MouseLook {
    pub sensitivity: f32,
    pub alpha: f32,
    pub true_dx: f32,
    pub true_dy: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl MouseLook {
    pub fn base(sensitivity: f32, alpha: f32) -> Self {
        Self {
            sensitivity,
            alpha,
            true_dx: 0.,
            true_dy: 0.,
            pitch: 0.,
            yaw: 0.,
        }
    }
}

#[system(for_each)]
fn player_movement(
    movement: &PlayerInput,
    dir: &Direction,
    pos: &mut Position,
    vel: &mut Velocity,
    #[resource] input: &Input,
    #[resource] time: &Time,
) {
    let mut movement_vec = Vector3::new(0.0, 0.0, 0.0);

    if input.pressed_keys.contains_key(&KeyCode::KeyW)
        || input.pressed_keys.contains_key(&KeyCode::ArrowUp)
    {
        movement_vec += dir.vector;
    }
    if input.pressed_keys.contains_key(&KeyCode::KeyS)
        || input.pressed_keys.contains_key(&KeyCode::ArrowDown)
    {
        movement_vec -= dir.vector;
    }
    if input.pressed_keys.contains_key(&KeyCode::KeyD)
        || input.pressed_keys.contains_key(&KeyCode::ArrowRight)
    {
        movement_vec += dir.vector.cross(Vector3::unit_y());
    }
    if input.pressed_keys.contains_key(&KeyCode::KeyA)
        || input.pressed_keys.contains_key(&KeyCode::ArrowLeft)
    {
        movement_vec -= dir.vector.cross(Vector3::unit_y());
    }
    if input.pressed_keys.contains_key(&KeyCode::Space) {
        vel.vector.y += movement.jump_vel;
    }

    movement_vec.y = 0.;
    if movement_vec != Vector3::zero() {
        // If movement_vec is 0, normalize will return NaNs
        pos.vector += movement_vec.normalize() * movement.speed * time.dt * 100.;
    }
}

#[system(for_each)]
fn look_around(
    look: &mut MouseLook,
    dir: &mut Direction,
    #[resource] input: &Input,
    #[resource] time: &Time,
) {
    look.true_dx =
        lerp(input.mouse_dx as f32, look.true_dx, look.alpha) * look.sensitivity * time.dt;
    look.true_dy =
        lerp(input.mouse_dy as f32, look.true_dy, look.alpha) * look.sensitivity * time.dt;

    look.yaw += look.true_dx as f32 * look.sensitivity;
    look.pitch += -look.true_dy as f32 * look.sensitivity;

    if look.pitch > 89.0 {
        look.pitch = 89.0
    }
    if look.pitch < -89.0 {
        look.pitch = -89.0
    }

    let xdir = look.yaw.to_radians().cos() * look.pitch.to_radians().cos();
    let ydir = look.pitch.to_radians().sin();
    let zdir = look.yaw.to_radians().sin() * look.pitch.to_radians().cos();

    dir.vector = Vector3::new(xdir, ydir, zdir).normalize();
}

#[system(for_each)]
fn place_block(
    pos: &Position,
    dir: &Direction,
    player: &PlayerInput,
    #[resource] blocks: &mut WorldBlocks,
    #[resource] input: &Input,
) {
    if input.mouse_buttons.get(&MouseButton::Right).is_some_and(|x| *x) {                                                               // On mouse right click
        if let Some((loc, block, face)) = blocks.get_raycast_intersect(pos.vector, player.reach, dir.vector) {  // If intersecting a block
            if blocks.get_block(loc + face).is_some_and(|block| block == 0) {                                                              // If block is loaded in and is air
                blocks.set_block(loc + face, 1);                                                                                               // Set the block equal to grass for now
            }
        }
    }
}

pub fn schedule(scheduler: &mut Builder) {
    scheduler.add_system(player_movement_system());
    scheduler.add_system(look_around_system());
    scheduler.add_system(place_block_system());
}
