mod texture2d;
mod depthtexture;
mod text_render;
mod camera;
mod projection;
pub mod cube_render;

use std::sync::Arc;
use std::time::Instant;

use anyhow;
use winit::{window::Window};
use cgmath::{Vector3, Vector4, Quaternion};

use crate::game::renderables::Renderables;
use cube_render::CubeRenderer;
use text_render::{ FontRenderer, sentence::Sentence, text_style::TextStyle } ;
use camera::{ Camera, CameraInitials };
use depthtexture::DepthTexture;


pub struct Graphics {
    // WGPU stuff
    surface: wgpu::Surface<'static>, // Represents the surface to be drawn on
    surface_format: wgpu::TextureFormat,
    device: wgpu::Device, // Represents phycical device
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    pub window: Arc<Window>,

    // My stuff
    depth_texture: DepthTexture,
    cube_renderer: CubeRenderer,
    font_renderer: FontRenderer,
    pub camera: Camera,

    last_frame: Instant,
    delta_time: f32,
}

impl Graphics {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let initials = CameraInitials {
            width: config.width as f32,
            height: config.height as f32,
            fovy: 90.0,
            znear: 0.1,
            zfar: 100.0
        };

        let camera = Camera::new(
            &device,
            initials,
            config.width,
            config.height
        );

        let depth_texture = DepthTexture::new(&device, &config);

        let cube_renderer = CubeRenderer::new(&device, &queue, config.format, &camera.bind_group_layout);

        let mut font_renderer = FontRenderer::new(&device, config.format, &camera.bind_group_layout);
        font_renderer.add_font(&device, &queue, "Arial", 100., include_bytes!("../../resources/fonts/arial.ttf"));

        Ok(Self {
            surface,
            surface_format,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            depth_texture,
            cube_renderer,
            font_renderer,
            camera,
            last_frame: Instant::now(),
            delta_time: 0., // We don't want anything using this until the first frame is rendered!
            // Not an option due to performance concerns
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            self.depth_texture = DepthTexture::new(&self.device, &self.config);
        }
    }

    fn render_inject(graphics: &mut Graphics, render_pass: &mut wgpu::RenderPass, renderables: Renderables) {
        graphics.cube_renderer.render(render_pass, &graphics.queue, &graphics.camera.bind_group, 
                                     &renderables.cubes);
        
        let mut sentences = Vec::new();

        sentences.push(Sentence {
            data: "Test text".to_owned(),
            position: Vector3::new(0.0, 0., 0.1),
            direction: Quaternion::new(1., 0., 0., 0.),
            text_style: TextStyle {
                font: "Arial".to_owned(),
                color: Vector4::new(1., 1., 1., 1.),
                scale: 1.,
                affected_by_camera: false
            }
        });

        
        sentences.push(Sentence {
            data: format!("{} FPS", 1. / graphics.delta_time),
            position: Vector3::new(-1.0, 0.9, 0.1),
            direction: Quaternion::new(1., 0., 0., 0.),
            text_style: TextStyle {
                font: "Arial".to_owned(),
                color: Vector4::new(1., 1., 1., 1.),
                scale: 2.,
                affected_by_camera: false
            }
        });
        

        graphics.font_renderer.render_sentences(sentences, render_pass, &graphics.queue, &graphics.camera.bind_group);
        
    }

    pub fn render(&mut self, renderables: Renderables) -> Result<(), wgpu::SurfaceError> {
        // Delta time
        self.delta_time = (Instant::now() - self.last_frame).as_secs_f32();
        self.last_frame = Instant::now();


        self.camera.update_camera(&self.queue, &renderables, self.config.width, self.config.height);

        if !self.is_surface_configured {  // Ensure that all WGPU processes are finished
            return Ok(());
        }

        // Generate required information
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {  // Block so that begin_render_pass can borrow encoder and give back
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {  // Being a list allows a pipeline to be made
                    view: &view,  // Where to save colors to
                    resolve_target: None,
                    ops: wgpu::Operations { // What to do with the colors on the screen
                        load: wgpu::LoadOp::Clear(wgpu::Color { // LoadOp::Clear: Get rid of the previous frmes
                            r: 0.1, // Tells it a single color for every single frame?
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,  // We store them because we do want our results to have an effect
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set: None,
                timestamp_writes: None
            });

            // Submit desired renders to render_pass
            Graphics::render_inject(self, &mut render_pass, renderables);
        }

        // Submit job to be rendered
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
            

    }
}


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);