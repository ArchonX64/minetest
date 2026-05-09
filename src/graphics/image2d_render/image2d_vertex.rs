use bytemuck;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Image2DVertex {
    position: [f32; 3],
    tex: [f32; 2]
}

impl Image2DVertex {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Image2DVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x2,
        ]
    };

    // First 3: Verticies
    // Last 2: Texture Coords
    pub const VERTICES: &[Image2DVertex] = &[
        Image2DVertex { position: [0.0, 1.0, 0.0], tex: [0.0, 0.0] }, // 0 TL
        Image2DVertex { position: [0.0, 0.0, 0.0], tex: [0.0, 1.0] }, // 1 BL
        Image2DVertex { position: [1.0, 0.0, 0.0], tex: [1.0, 1.0] }, // 2 BR
        Image2DVertex { position: [1.0, 1.0, 0.0], tex: [1.0, 0.0] }, // 3 TR
    ];

    pub const INDEXES: &[u32] = &[
        0, 1, 2,  // TL, BL, BR  (CCW)
        0, 2, 3,  // TL, BR, TR  (CCW)
    ];
}

