use bytemuck;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FontVertex {
    position: [f32; 3],
    tex: [f32; 2]
}

impl FontVertex {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<FontVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x2,
        ]
    };
}

// First 3: Verticies
// Last 2: Texture Coords
pub const FONT_VERTICES: &[FontVertex] = &[
    FontVertex { position: [0.0, 1.0, 0.0], tex: [0.0, 0.0] }, // 0 TL
    FontVertex { position: [0.0, 0.0, 0.0], tex: [0.0, 1.0] }, // 1 BL
    FontVertex { position: [1.0, 0.0, 0.0], tex: [1.0, 1.0] }, // 2 BR
    FontVertex { position: [1.0, 1.0, 0.0], tex: [1.0, 0.0] }, // 3 TR
];

pub const FONT_INDEXES: &[u32] = &[
    0, 1, 2,  // TL, BL, BR  (CCW)
    0, 2, 3,  // TL, BR, TR  (CCW)
];