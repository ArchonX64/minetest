pub mod image2d_instance;
pub mod image2d_vertex;

use std::collections::HashMap;

use wgpu::{ Buffer, BufferDescriptor, BufferUsages, Device, PipelineCompilationOptions, PipelineLayoutDescriptor, Queue, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule, TextureFormat, VertexBufferLayout, VertexState, include_wgsl, util::{BufferInitDescriptor, DeviceExt} };

use crate::graphics::{image2d_render::image2d_instance::Image2DInstance, texture2d::Texture2D};
use super::image2d_render::{ image2d_instance::Image2DInstanceRaw, image2d_vertex::Image2DVertex };


pub struct Image2DRenderer {
    shader: ShaderModule,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
    pipeline: RenderPipeline
}

impl Image2DRenderer {
    const MAX_INSTANCES: usize = 64;

    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("image2d.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Image2DRenderer Render Pipeline Layout"),
            bind_group_layouts: &[&Texture2D::get_layout(device, "Image2DRenderer Texture Bind Group Layout")],
            push_constant_ranges: &[]
        });

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Image2DRenderer Vertex Buffer"),
            contents: bytemuck::cast_slice(Image2DVertex::VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image2DRenderer Index Buffer"),
            contents: bytemuck::cast_slice(Image2DVertex::INDEXES),
            usage: wgpu::BufferUsages::INDEX
        });

        let instance_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Image2DRenderer Instance Buffer"),
            size: (std::mem::size_of::<Image2DInstanceRaw>() * Self::MAX_INSTANCES) as u64,
            usage: BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Image2DRenderer Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Image2DVertex::LAYOUT, Image2DInstance::LAYOUT],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture2D::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(), // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
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
            index_buffer,
            instance_buffer,
            pipeline,
        }
    }

    pub fn render(&self, render_pass: &RenderPass, queue: &Queue, instances: Vec<Image2DInstance>) {
        // Group by shared texture
        let textures: HashMap<Texture2D, Vec<Image2DInstanceRaw>> = HashMap::new();
        //for instance in instances {
            //textures.entry(instance.texture).and_modify(|x| x.appen)
        //}
    }
}