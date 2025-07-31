use cgmath::{ self, Point3, Vector3 };
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
    view_proj: [[f32; 4]; 4]
}

impl Camera {
    pub fn new(device: &wgpu::Device, initials: CameraInitials) -> Self {
        
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
            proj,
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    fn build_view_projection_matrix(&self, position: Point3<f32>, direction: Vector3<f32>) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(
            position,
            position + direction,
            cgmath::Vector3::unit_y());

        return super::OPENGL_TO_WGPU_MATRIX * self.proj.calc_matrix() * view;
    }

    pub fn update_camera(&mut self, queue: &wgpu::Queue, renderables: &Renderables) {
        self.uniform.view_proj = self.build_view_projection_matrix(renderables.cam_pos, renderables.cam_dir).into();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
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