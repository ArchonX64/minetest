use wgpu::{ VertexBufferLayout, BufferAddress, VertexStepMode, vertex_attr_array };
use cgmath::{ Vector2 };

use crate::graphics::texture2d::Texture2D;

pub struct Image2DInstance {
    pub texture: Texture2D,
    pub position: Vector2<f32>,
    pub scale: f32
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Image2DInstanceRaw {
    position: [f32; 2],
    scale: f32,
}

impl Image2DInstance {
    pub fn to_raw(&self) -> Image2DInstanceRaw {
        Image2DInstanceRaw {
            position: self.position.into(),
            scale: self.scale,
        }
    }

    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: std::mem::size_of::<Image2DInstanceRaw>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        attributes: &vertex_attr_array![
            3 => Float32x2,
            4 => Float32,
        ]
    };
}