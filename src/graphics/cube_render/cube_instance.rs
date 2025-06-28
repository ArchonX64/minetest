
pub struct CubeInstance {
    pub tex_index: u32,
    pub position: cgmath::Point3<f32>
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CubeInstanceRaw {
    tex_index: f32,
    location: [f32; 3],
}

impl CubeInstance {
    pub fn to_raw(&self) -> CubeInstanceRaw {
        CubeInstanceRaw {
            tex_index: self.tex_index as f32,
            location: self.position.into(),
        }
    }
}

impl CubeInstanceRaw { 
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<CubeInstanceRaw>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &wgpu::vertex_attr_array![
            3 => Float32,
            4 => Float32x3
        ]
    };
}
