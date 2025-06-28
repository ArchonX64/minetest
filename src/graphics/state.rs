use std::sync::Arc;
use std::time::Instant;

use anyhow;
use winit::{window::Window, keyboard::KeyCode};

use crate::application::Input;
use crate::graphics::camera::CameraInitials;

use super::cube_render::{ CubeRenderer, cube_instance::CubeInstance };
use super::camera::Camera;


pub struct State {
    // WGPU stuff
    surface: wgpu::Surface<'static>, // Represents the surface to be drawn on
    device: wgpu::Device, // Represents phycical device
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    pub window: Arc<Window>,

    // My stuff
    cube_renderer: CubeRenderer,
    pub camera: Camera,
    last_frame: Instant,
    delta_time: f32,
    cubes: Vec<CubeInstance>
}

impl State {
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
            pos: (1.0, 0.0, 0.0).into(),
            direction: (-1.0, 0.0, 0.0).into(),
            pitch: 0.,
            yaw: 0.,
            speed: 1.0,
            sensitivity: 0.5,
            width: config.width as f32,
            height: config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0
        };

        let camera = Camera::new(
            &device,
            &queue,
            initials,
        );

        let cube_renderer = CubeRenderer::new(&device, &queue, config.format, &camera.bind_group_layout);

        let cubes: Vec<CubeInstance> = vec![
            CubeInstance { tex_index: 0, position: cgmath::Point3 { x: 1., y: 1., z: 1. }},
            CubeInstance { tex_index: 0, position: cgmath::Point3 { x: 2., y: 1., z: 1. }},
            CubeInstance { tex_index: 0, position: cgmath::Point3 { x: 3., y: 1., z: 1. }},
            CubeInstance { tex_index: 0, position: cgmath::Point3 { x: 1., y: 1., z: 2. }},
            CubeInstance { tex_index: 0, position: cgmath::Point3 { x: 1., y: 1., z: 3. }}
        ];

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            cube_renderer,
            camera,
            last_frame: Instant::now(),
            delta_time: 0., // We don't want anything using this until the first frame is rendered!
            // Not an option due to performance concerns
            cubes
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
            if width > 0 && height > 0 {
                self.config.width = width;
                self.config.height = height;
                self.surface.configure(&self.device, &self.config);
                self.is_surface_configured = true;
            }
    }

    pub fn render(&mut self, input: Input) -> Result<(), wgpu::SurfaceError> {
        // Delta time
        self.delta_time = (Instant::now() - self.last_frame).as_secs_f32();
        self.last_frame = Instant::now();

        self.camera.update_camera(&self.queue, &input, self.delta_time);

        // REQUIRES RENDER_PASS
        wgpu_render(self, |state, render_pass | {
            state.cube_renderer.render(render_pass, &state.queue, &state.camera.bind_group, &state.cubes);
        })

    }

    pub fn on_key_press(&mut self, code: KeyCode) {
        
    }

    pub fn on_key_release(&mut self, code: KeyCode) {

    }

    pub fn on_mouse_move(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.camera.change_direction(mouse_dx, mouse_dy);
    }

    pub fn update(&mut self) {

    }


}

// Renders all drawings to screen, resulting in a new frame
    fn wgpu_render<F>(state: &mut State, rendering: F) -> Result<(), wgpu::SurfaceError> where
        F: Fn(&State, &mut wgpu::RenderPass)
    {
        state.window.request_redraw(); // Tell the window object to prepare for redrawing

        if !state.is_surface_configured {  // Ensure that all WGPU processes are finished
            return Ok(());
        }

        // Generate required information
        let output = state.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });

            // Submit desired renders to render_pass
            rendering(&state, &mut render_pass);
        }

        // Submit job to be rendered
        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }