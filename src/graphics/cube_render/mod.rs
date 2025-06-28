mod cube_vertex;
pub mod cube_instance;

use wgpu::util::DeviceExt;

use super::texture2d::Texture2D;
use cube_vertex::{CubeVertex, CUBE_VERTICES};
use cube_instance::{ CubeInstance, CubeInstanceRaw };

pub struct CubeRenderer {
    shader: wgpu::ShaderModule,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    texture_map: Texture2D
}

impl CubeRenderer {
    const MAX_INSTANCES: u64 = 100;

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat, camera_layout: &wgpu::BindGroupLayout) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("cube.wgsl"));
        let texture_map = Texture2D::new("Some Texture", &device, &queue,
         include_bytes!("../../../resources/textures/grass_full.png"));

        let render_pipeline_layout = 
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Descriptor"),
                bind_group_layouts: &[camera_layout, &texture_map.bind_group_layout],
                push_constant_ranges: &[]
            });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Cube Instance Buffer"),
            size: (std::mem::size_of::<f32>() as u64) * 4 * Self::MAX_INSTANCES,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[CubeVertex::LAYOUT, CubeInstanceRaw::LAYOUT],
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

        Self {
            shader,
            vertex_buffer,
            render_pipeline,
            texture_map,
            instance_buffer
        }
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue, camera: &wgpu::BindGroup,
        instances: &Vec<CubeInstance>) {

        // Update instances
        let raw = instances.iter().map(|x| x.to_raw()).collect::<Vec<_>>();
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&raw));

        render_pass.set_pipeline(&self.render_pipeline);

        // Bind Groups
        render_pass.set_bind_group(0, camera, &[]); // Texture
        render_pass.set_bind_group(1, &self.texture_map.bind_group, &[]);  // Camera Uniform

        // Vertex Buffer
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        // Drawing
        render_pass.draw(0..CUBE_VERTICES.len() as u32, 0..instances.len() as u32);
    }
}


