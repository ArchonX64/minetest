use image;

use wgpu::util::DeviceExt;

use super::texture2d::Texture2D;
use super::state::State;

pub struct CubeRenderer {
    shader: wgpu::ShaderModule,
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    num_verts: u32,

    // Textures
    texture_map: Texture2D
}

struct CubeInstance {
    tex_index: u32,
    position: cgmath::Point3<u32>
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CubeInstanceRaw {
    tex_index: u32,
    location: [u32; 3],
    _buffer: u32
}

impl CubeInstance {
    pub fn to_raw(&self) -> CubeInstanceRaw {
        CubeInstanceRaw {
            tex_index: self.tex_index,
            location: self.position.into(),
            _buffer: 0
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CubeVertex {
    position: [f32; 3],
    texture: [f32; 2],
    tex_offset: [f32; 1]
}

impl CubeVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CubeVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl CubeRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat, bind_groups: &[&wgpu::BindGroupLayout]) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../../resources/shaders/cube.wgsl"));
        let texture_map = Texture2D::new("Some Texture", &device, &queue,
         include_bytes!("../../resources/textures/grass_full.png"));

        let mut bind_groups_combined = [bind_groups].concat();
        bind_groups_combined.insert(0, &texture_map.bind_group_layout);
        let bind_groups_slice: &[&wgpu::BindGroupLayout] = &bind_groups_combined;

        let render_pipeline_layout = 
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Descriptor"),
                bind_group_layouts: bind_groups_slice,
                push_constant_ranges: &[]
            });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[CubeVertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let num_verts = std::mem::size_of::<CubeVertex>() as u32;

        Self {
            shader,
            vertex_buffer,
            render_pipeline,
            num_verts,
            texture_map
        }
    }

    pub fn render(&self, state: &State, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);

        // Bind Groups
        render_pass.set_bind_group(0, &self.texture_map.bind_group, &[]); // Texture
        render_pass.set_bind_group(1, &state.camera.bind_group, &[]);  // Camera Uniform

        // Vertex Buffer
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // Drawing
        render_pass.draw(0..CUBE_VERTICES.len() as u32, 0..1);
    }
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

