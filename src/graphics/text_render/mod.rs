mod character;
mod font;
mod font_vertex;
mod font_instance;
pub mod text_style;
pub mod sentence;

use font_vertex::{ FontVertex, FONT_VERTICES, FONT_INDEXES };
use font_instance::{ FontInstance, FontInstanceRaw };
use character::FontCharacter;
use font::FontData;
use sentence::Sentence;

use super::texture2d::Texture2D;

use std::{collections::HashMap};

use itertools::Itertools;
use cgmath::{ ElementWise, Vector2, Vector3, Zero };
use wgpu::{ Device, Queue, BindGroupLayout, util::DeviceExt };
use ab_glyph::{ Font, FontRef, GlyphId, OutlinedGlyph, PxScaleFont, ScaleFont };
use rect_packer::Packer;
//use image::{ImageBuffer, Rgba};

pub struct FontRenderer {
    fonts: HashMap<String, FontData>,

    render_pipeline: wgpu::RenderPipeline,
    shader: wgpu::ShaderModule,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
}

impl FontRenderer {
    pub const FONT_ATLAS_SIZE: usize = 1024;
    pub const RGBA_SIZE: usize = 4;
    pub const MAX_INSTANCES: usize = 1000;
    
    pub fn new(device: &Device, format: wgpu::TextureFormat, camera_layout: &BindGroupLayout) -> Self {
        // -- SHADER INIT --
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // -- BUFFER INIT --
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Font Vertex Buffer"),
            contents: bytemuck::cast_slice(FONT_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Font Index Buffer"),
            contents: bytemuck::cast_slice(FONT_INDEXES),
            usage: wgpu::BufferUsages::INDEX
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Font Instance Buffer"),
            size: (std::mem::size_of::<FontInstanceRaw>() * Self::MAX_INSTANCES) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // -- RENDER PIPELINE --
        let render_pipeline_layout = 
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Font Render Pipeline Layout"),
                bind_group_layouts: &[camera_layout, &Texture2D::get_layout(device, "Font Atlas Bind Group Layout")],
                push_constant_ranges: &[]
            });
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[FontVertex::LAYOUT, FontInstanceRaw::LAYOUT],
                compilation_options: wgpu::PipelineCompilationOptions::default()
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
            fonts: HashMap::new(),
            render_pipeline,
            shader,
            vertex_buffer,
            index_buffer,
            instance_buffer
        }
    }

    pub fn add_font(&mut self, device: &Device, queue: &Queue, font_name: &str, font_scale: f32, font_data: &'static [u8]) {
        let font = FontRef::try_from_slice(font_data).unwrap();
        let scaled_font = font.as_scaled(font_scale);

        let mut packer = Packer::new(rect_packer::Config {
            width: Self::FONT_ATLAS_SIZE as i32,
            height: Self::FONT_ATLAS_SIZE as i32,
            border_padding: 5,
            rectangle_padding: 10
        });

        // Atlas is 2D packing of all textures, 4 u8s for each for RGBA
        let mut text_data = vec![0u8; Self::FONT_ATLAS_SIZE * Self::FONT_ATLAS_SIZE * Self::RGBA_SIZE];

        let mut glyphs: HashMap<char, FontCharacter> = HashMap::new();

        // Create structs with character data, push atlas into text_data
        for (id, char) in scaled_font.font.codepoint_ids() {
            let glyph = scaled_font.scaled_glyph(char);

            match scaled_font.outline_glyph(glyph) {
                Some(outline) => self.pack_glpyh(&mut packer, scaled_font, &outline, char, id, &mut text_data, &mut glyphs),
                None => {
                    glyphs.insert(char, FontCharacter { // Glyphs with no content (whitespace) have no outline
                        position: Vector2::new(0., 0.),
                        size: Vector2::zero(),
                        bearing: Vector2::zero(),
                        advance: scaled_font.h_advance(id)
                    });
                }
            }
        }

        // To test the packing
        //let buffer: ImageBuffer<Rgba<u8>, _> 
        //        = ImageBuffer::from_raw(Self::FONT_ATLAS_SIZE as u32, Self::FONT_ATLAS_SIZE as u32, text_data.clone())
        //        .expect("Failed to get image data!");
        //buffer.save(Path::new("test.png"));
            
        let label = font_name.to_owned();
        let texture = Texture2D::from_bytes(&label, device, queue, (Self::FONT_ATLAS_SIZE as u32, Self::FONT_ATLAS_SIZE as u32), &text_data,
                                            wgpu::FilterMode::Linear);
        
        self.fonts.insert(font_name.to_owned(), FontData {
            texture,
            glyphs
        });
    }

    fn pack_glpyh(&mut self, packer: &mut Packer, scaled_font: PxScaleFont<&FontRef<'_>>, outline: &OutlinedGlyph, char: char,
                  id: GlyphId, text_data: &mut Vec<u8>, glyphs: &mut HashMap<char, FontCharacter>) {
        let (width, height) = (outline.px_bounds().width() as i32, outline.px_bounds().height() as i32);
            
        if let Some(rect) = packer.pack(width, height, false) {
            glyphs.insert(char, FontCharacter {
                position: Vector2 { x: rect.x as f32, y: rect.y as f32 },
                size: Vector2 { x: width as f32, y: height as f32},
                bearing: Vector2 { x: scaled_font.h_side_bearing(id), y: outline.px_bounds().max.y},
                advance: scaled_font.h_advance(id),
            });

            outline.draw(|x, y, c| {
                let index = ((rect.x as u32 + x) + (rect.y as u32 + y) * Self::FONT_ATLAS_SIZE as u32) as usize;

                text_data[index * Self::RGBA_SIZE + 0] = 255;
                text_data[index * Self::RGBA_SIZE + 1] = 255;
                text_data[index * Self::RGBA_SIZE + 2] = 255;
                text_data[index * Self::RGBA_SIZE + 3] = (c * 255.0).clamp(0.0, 255.0) as u8; // If its larger than 1, multiplying it by 255 will overflow
            });
        }
    }

    pub fn render_sentences(&self, sentences: Vec<Sentence>, render_pass: &mut wgpu::RenderPass,
                         queue: &Queue, camera: &wgpu::BindGroup) {
        let mut instances = Vec::new();
        for sentence_data in sentences {
            let mut advance = 0.;
            let mut sentence_drawn = Vec::new();
            let font = &self.fonts.get(&sentence_data.text_style.font).unwrap();

            // Create unscaled version first
            for letter in sentence_data.data.chars() {
                let glyph = font.glyphs.get(&letter).unwrap();

                let instance = FontInstance {
                    sentence_position: sentence_data.position,
                    letter_position: Vector3::new(advance + glyph.bearing.x, -glyph.bearing.y, 0.),
                    direction: sentence_data.direction,
                    tex_offset: glyph.position,
                    tex_size: glyph.size,
                    text_style: sentence_data.text_style.clone()
                };

                println!("letter={} bearing_y={}", letter, glyph.bearing.y);
                
                sentence_drawn.push(instance);

                advance += glyph.advance;
            }

            instances.append(&mut sentence_drawn);
        }
        self.render(render_pass, queue, camera, instances);
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass, queue: &Queue, camera: &wgpu::BindGroup,
                 instances: Vec<FontInstance>) {

        // Prepare buffer and pipeline
        let instances_sorted = instances.iter()
            .sorted_by(|a, b| a.text_style.font.cmp(&b.text_style.font))
            .collect::<Vec<_>>();

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, camera, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        // Determine which fonts and how many
        let mut num_fonts = HashMap::new();
        for font in instances_sorted.iter().map(|x| &x.text_style.font) {
            *num_fonts.entry(font).or_insert(0) += 1;
        }

        // Create raw data and send to gpu
        let raw = instances_sorted.iter()
            .map(|x| x.to_raw())
            .collect::<Vec<_>>();

        //raw.iter().for_each(|x| println!("{:.?}", x));

        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&raw));

        // Render each individual font glyph
        let mut prev = 0;
        for (font, size) in num_fonts {
            let font = self.fonts.get(font).unwrap(); // Verified on insertion
            
            render_pass.set_bind_group(1, &font.texture.bind_group, &[]);

            render_pass.draw_indexed(0..FONT_INDEXES.len() as u32, 0, prev..prev + size);

            prev += size
        };
    }
}

