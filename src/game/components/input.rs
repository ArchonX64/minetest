use winit::keyboard::KeyCode;
use legion::{ system, systems::Builder };
use cgmath::{ Vector3, InnerSpace, Zero };

use crate::game::components::spatial::Velocity;
use crate::{application::Input, util::lerp};
use super::spatial::{ Position, Direction };
use super::time::Time;

pub struct HumanoidKeyboardMovement {
    pub speed: f32,
    pub jump_vel: f32,
}

pub struct MouseLook {
    pub sensitivity: f32,
    pub alpha: f32,
    pub true_dx: f32,
    pub true_dy: f32,
    pub pitch: f32,
    pub yaw: f32
}

impl MouseLook {
    pub fn base(sensitivity: f32, alpha: f32) -> Self {
        Self { sensitivity, alpha, true_dx: 0., true_dy: 0., pitch: 0., yaw: 0. }
    }
}

#[system(for_each)]
fn player_movement(movement: &HumanoidKeyboardMovement, dir: &Direction, pos: &mut Position, vel: &mut Velocity,
     #[resource] input: &Input, #[resource] time: &Time) {
    let mut movement_vec = Vector3::new(0.0, 0.0, 0.0);

    if input.pressed_keys.contains(&KeyCode::KeyW) || input.pressed_keys.contains(&KeyCode::ArrowUp) {
        movement_vec += dir.vector;
    }
    if input.pressed_keys.contains(&KeyCode::KeyS) || input.pressed_keys.contains(&KeyCode::ArrowDown) {
        movement_vec -= dir.vector;
    }
    if input.pressed_keys.contains(&KeyCode::KeyD) || input.pressed_keys.contains(&KeyCode::ArrowRight) {
        movement_vec += dir.vector.cross(Vector3::unit_y());
    }
    if input.pressed_keys.contains(&KeyCode::KeyA) || input.pressed_keys.contains(&KeyCode::ArrowLeft) {
        movement_vec -= dir.vector.cross(Vector3::unit_y());
    }
    if input.pressed_keys.contains(&KeyCode::Space) {
        vel.vector.y += movement.jump_vel;
    }

    movement_vec.y = 0.;
    if movement_vec != Vector3::zero() { // If movement_vec is 0, normalize will return NaNs
        pos.vector += movement_vec.normalize() * movement.speed * time.dt * 100.;
    }
}

#[system(for_each)]
fn look_around(look: &mut MouseLook, dir: &mut Direction, #[resource] input: &Input, #[resource] time: &Time) {
    look.true_dx = lerp(input.mouse_dx as f32, look.true_dx, look.alpha) * look.sensitivity * time.dt;
    look.true_dy = lerp(input.mouse_dy as f32, look.true_dy, look.alpha) * look.sensitivity * time.dt;

    look.yaw += look.true_dx as f32 * look.sensitivity;
    look.pitch += -look.true_dy as f32 * look.sensitivity;

    if look.pitch > 89.0 { look.pitch = 89.0 }
    if look.pitch < -89.0 { look.pitch = -89.0 }

    let xdir = look.yaw.to_radians().cos() * look.pitch.to_radians().cos();
    let ydir = look.pitch.to_radians().sin();
    let zdir = look.yaw.to_radians().sin() * look.pitch.to_radians().cos();

    dir.vector = Vector3::new(xdir, ydir, zdir).normalize();
}

pub fn schedule(scheduler: &mut Builder) {
    scheduler.add_system(player_movement_system());
    scheduler.add_system(look_around_system());
}