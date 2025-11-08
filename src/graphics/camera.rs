use cgmath::{ self, Point3, Vector2, Vector3, Zero };
use bytemuck;
use wgpu::util::DeviceExt;

use crate::game::renderables::Renderables;

use super::projection::Projection;


pub struct Camera {
    uniform: CameraUniform,
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,

    proj: Projection,
    screen_size: Vector2<u32>
}

pub struct CameraInitials {
    pub width: f32,
    pub height: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    screen_size: [f32; 2],
    _buffer: [f32; 6]
}

impl Camera {
    pub fn new(device: &wgpu::Device, initials: CameraInitials, screen_width: u32, screen_height: u32) -> Self {
        
        let uniform = CameraUniform::new(screen_width, screen_height);

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

        let screen_size = Vector2::new(screen_width, screen_height);

        Self {
            proj,
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
            screen_size,
        }
    }

    fn build_view_projection_matrix(&self, position: Point3<f32>, direction: Vector3<f32>) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(
            position,
            position + direction,
            cgmath::Vector3::unit_y());

        return super::OPENGL_TO_WGPU_MATRIX * self.proj.calc_matrix() * view;
    }

    pub fn update_camera(&mut self, queue: &wgpu::Queue, renderables: &Renderables, screen_width: u32, screen_height: u32) {
        self.uniform.view_proj = self.build_view_projection_matrix(renderables.cam_pos, renderables.cam_dir).into();
        self.screen_size.x = screen_width;
        self.screen_size.y = screen_height;
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

impl CameraUniform {
    fn new(screen_width: u32, screen_height: u32) -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            screen_size: [screen_width as f32, screen_height as f32],
            _buffer: [0.; 6]
        }
    }
}