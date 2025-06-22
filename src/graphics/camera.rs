use cgmath::{self, Angle, InnerSpace, Zero};
use bytemuck;
use wgpu::util::DeviceExt;
use winit::keyboard::KeyCode;

use crate::application::Input;

use super::projection::Projection;


pub struct Camera {
    // Mathy stuff
    pos: cgmath::Point3<f32>,
    direction: cgmath::Vector3<f32>,
    pitch: f32,
    yaw: f32,
    speed: f32,
    sensitivity: f32,

    // Graphicy stuff
    uniform: CameraUniform,
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,

    proj: Projection,
}

pub struct CameraInitials {
    pub pos: cgmath::Point3<f32>,
    pub direction: cgmath::Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub width: f32,
    pub height: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4]
}

impl Camera {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, initials: CameraInitials) -> Self {
        
        let uniform = CameraUniform::new();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
        });

        let proj = Projection::new(&initials);

        Self {
            pos: initials.pos,
            direction: initials.direction,
            pitch: initials.pitch,
            yaw: initials.yaw,
            speed: initials.speed,
            sensitivity: initials.sensitivity,
            proj,
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {

        let view = cgmath::Matrix4::look_at_rh(
            self.pos,
            self.pos + self.direction,
            cgmath::Vector3::unit_y());

        return super::OPENGL_TO_WGPU_MATRIX * self.proj.calc_matrix() * view;
    }

    pub fn update_camera(&mut self, queue: &wgpu::Queue, input: &Input, delta_time: f32) {
        self.movement(&input.pressed_keys, delta_time);

        // Ready uniform
        self.uniform.view_proj = self.build_view_projection_matrix().into();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

    fn movement(&mut self, pressed_keys: &Vec<KeyCode>, delta_time: f32) {
        let mut movement_vec= cgmath::Vector3::new(0.0, 0.0, 0.0);

        if pressed_keys.contains(&KeyCode::KeyW) || pressed_keys.contains(&KeyCode::ArrowUp) {
            movement_vec += self.direction;
        }
        if pressed_keys.contains(&KeyCode::KeyS) || pressed_keys.contains(&KeyCode::ArrowDown) {
            movement_vec -= self.direction;
        }
        if pressed_keys.contains(&KeyCode::KeyD) || pressed_keys.contains(&KeyCode::ArrowRight) {
            movement_vec += self.direction.cross(cgmath::Vector3::unit_y());
        }
        if pressed_keys.contains(&KeyCode::KeyA) || pressed_keys.contains(&KeyCode::ArrowLeft) {
            movement_vec -= self.direction.cross(cgmath::Vector3::unit_y());
        }
        if pressed_keys.contains(&KeyCode::Space) {
            movement_vec += cgmath::Vector3::unit_y(); 
        }
        if pressed_keys.contains(&KeyCode::ShiftLeft) {
            movement_vec -= cgmath::Vector3::unit_y();
        }

        if movement_vec != cgmath::Vector3::zero() { // If movement_vec is 0, normalize will return NaNs
            self.pos += movement_vec.normalize() * self.speed * delta_time;
        }
    }

    pub fn change_direction(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.yaw += mouse_dx as f32 * self.sensitivity;
        self.pitch -= mouse_dy as f32 * self.sensitivity;

        if self.pitch > 89.0 { self.pitch = 89.0 }
        if self.pitch < -89.0 { self.pitch = -89.0 }

        let xdir = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let ydir = self.pitch.to_radians().sin();
        let zdir = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.direction = cgmath::Vector3::new(xdir, ydir, zdir).normalize();
    }
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}