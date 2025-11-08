use cgmath::{ Vector2, Vector3, Point2, Quaternion };

use super::text_style::TextStyle;

use super::super::text_render::FontRenderer;

#[derive(Debug)]
pub struct FontInstance {
    pub sentence_position: Vector3<f32>,
    pub letter_position: Vector3<f32>,
    pub direction: Quaternion<f32>,
    pub tex_offset: Vector2<f32>,
    pub tex_size: Vector2<f32>,
    pub text_style: TextStyle,
}

impl FontInstance {
    pub fn to_raw(&self) -> FontInstanceRaw{
        FontInstanceRaw::new(self)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FontInstanceRaw {
    pub sentence_position: [f32; 3],
    pub letter_position: [f32; 3],
    pub direction: [f32; 4],
    pub tex_offset: [f32; 2],
    pub tex_size: [f32; 2],
    pub color: [f32; 4],
    pub scale: f32,
    pub affected_by_camera: u32
}

impl FontInstanceRaw {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<FontInstanceRaw>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &wgpu::vertex_attr_array![
            2 => Float32x3,
            3 => Float32x3,
            4 => Float32x4,
            5 => Float32x2,
            6 => Float32x2,
            7 => Float32x4,
            8 => Float32,
            9 => Uint32,
        ]
    };

    pub fn new(i: &FontInstance) -> Self {
        Self {
            sentence_position: i.sentence_position.into(),
            letter_position: i.letter_position.into(),
            direction: i.direction.into(),
            tex_offset: i.tex_offset.into(),
            tex_size: i.tex_size.into(),
            color: i.text_style.color.into(),
            scale: i.text_style.scale,
            affected_by_camera: i.text_style.affected_by_camera.into()
        }
    }
}