

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CubeVertex {
    position: [f32; 3],
    texture: [f32; 2],
    tex_offset: [f32; 1]
}

impl CubeVertex {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<CubeVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x2,
            2 => Float32
        ]
    };
}

pub const CUBE_VERTICES: &[CubeVertex] = &[
    CubeVertex { position: [ 0.5, -0.5, -0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [0.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5, -0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [0.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5, -0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [0.0 / 6.0] },
    CubeVertex { position: [-0.5,  0.5, -0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [0.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5, -0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [0.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5, -0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [0.0 / 6.0] }, // Front
    CubeVertex { position: [-0.5, -0.5,  0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [2.0 / 6.0] },
    CubeVertex { position: [ 0.5, -0.5,  0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [2.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5,  0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [2.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5,  0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [2.0 / 6.0] },
    CubeVertex { position: [-0.5,  0.5,  0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [2.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5,  0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [2.0 / 6.0] }, // Back
    CubeVertex { position: [-0.5,  0.5,  0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [3.0 / 6.0] },
    CubeVertex { position: [-0.5,  0.5, -0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [3.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5, -0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [3.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5, -0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [3.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5,  0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [3.0 / 6.0] },
    CubeVertex { position: [-0.5,  0.5,  0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [3.0 / 6.0] }, // Left
    CubeVertex { position: [ 0.5,  0.5, -0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [1.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5,  0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [1.0 / 6.0] },
    CubeVertex { position: [ 0.5, -0.5, -0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [1.0 / 6.0] },
    CubeVertex { position: [ 0.5, -0.5,  0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [1.0 / 6.0] },
    CubeVertex { position: [ 0.5, -0.5, -0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [1.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5,  0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [1.0 / 6.0] }, // Right
    CubeVertex { position: [-0.5, -0.5, -0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [5.0 / 6.0] },
    CubeVertex { position: [ 0.5, -0.5, -0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [5.0 / 6.0] },
    CubeVertex { position: [ 0.5, -0.5,  0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [5.0 / 6.0] },
    CubeVertex { position: [ 0.5, -0.5,  0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [5.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5,  0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [5.0 / 6.0] },
    CubeVertex { position: [-0.5, -0.5, -0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [5.0 / 6.0] }, // Bottom
    CubeVertex { position: [ 0.5,  0.5, -0.5], texture: [1.0 / 6.0, 0.0], tex_offset: [4.0 / 6.0] },
    CubeVertex { position: [-0.5,  0.5, -0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [4.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5,  0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [4.0 / 6.0] },
    CubeVertex { position: [-0.5,  0.5,  0.5], texture: [0.0 / 6.0, 1.0], tex_offset: [4.0 / 6.0] },
    CubeVertex { position: [ 0.5,  0.5,  0.5], texture: [1.0 / 6.0, 1.0], tex_offset: [4.0 / 6.0] },
    CubeVertex { position: [-0.5,  0.5, -0.5], texture: [0.0 / 6.0, 0.0], tex_offset: [4.0 / 6.0] } // Top
];

