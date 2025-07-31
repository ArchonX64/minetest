use crate::application::Input;
use crate::util::lerp;
use super::units::{ EntityCoords, PlayerDirection };

use cgmath::{ Vector3, Point3, Zero, InnerSpace };
use winit::keyboard::KeyCode;


pub struct Player {
    pitch: f32, // Camera Stuff
    yaw: f32,
    true_dx: f32,
    true_dy: f32,
    look_alpha: f32,
    look_sensitivity: f32,

    speed: f32,
    pub position: EntityCoords,
    pub direction: PlayerDirection,
}

impl Player {
    const PLAYER_COLLIDER_SIZE: Vector3<f32> = Vector3::new(1., 2., 1.);
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            pitch: 0.0,
            yaw: 0.0,
            true_dx: 0.0,
            true_dy: 0.0,
            look_alpha: 0.5,
            look_sensitivity: 10.0,
            position: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::zero(),
        }
    }

    fn movement(&mut self, dt: f32, input: &Input) {
        let mut movement_vec= Vector3::new(0.0, 0.0, 0.0);

        if input.pressed_keys.contains(&KeyCode::KeyW) || input.pressed_keys.contains(&KeyCode::ArrowUp) {
            movement_vec += self.direction;
        }
        if input.pressed_keys.contains(&KeyCode::KeyS) || input.pressed_keys.contains(&KeyCode::ArrowDown) {
            movement_vec -= self.direction;
        }
        if input.pressed_keys.contains(&KeyCode::KeyD) || input.pressed_keys.contains(&KeyCode::ArrowRight) {
            movement_vec += self.direction.cross(Vector3::unit_y());
        }
        if input.pressed_keys.contains(&KeyCode::KeyA) || input.pressed_keys.contains(&KeyCode::ArrowLeft) {
            movement_vec -= self.direction.cross(Vector3::unit_y());
        }
        if input.pressed_keys.contains(&KeyCode::Space) {
            movement_vec += Vector3::unit_y(); 
        }
        if input.pressed_keys.contains(&KeyCode::ShiftLeft) {
            movement_vec -= Vector3::unit_y();
        }

        if movement_vec != Vector3::zero() { // If movement_vec is 0, normalize will return NaNs
            self.position += movement_vec.normalize() * self.speed * dt;
        }
    }

    fn look(&mut self, dt: f32, input: &Input) {
        self.true_dx = lerp(input.mouse_dx as f32, self.true_dx, self.look_alpha) * self.look_sensitivity * dt;
        self.true_dy = lerp(input.mouse_dy as f32, self.true_dy, self.look_alpha) * self.look_sensitivity * dt;

        self.yaw += self.true_dx as f32 * self.look_sensitivity;
        self.pitch += -self.true_dy as f32 * self.look_sensitivity;

        if self.pitch > 89.0 { self.pitch = 89.0 }
        if self.pitch < -89.0 { self.pitch = -89.0 }

        let xdir = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let ydir = self.pitch.to_radians().sin();
        let zdir = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.direction = Vector3::new(xdir, ydir, zdir).normalize();
    }
}