use cgmath::{ Quaternion, Point3, Vector3, Vector4, Zero };
use std::time::{ Duration, Instant };

use crate::graphics::texture2d::Texture2D;

use super::{ Graphics, Renderables, text_render::{ sentence::Sentence, text_style::TextStyle }};

pub struct InGameGUI {
    metric_style: TextStyle,
    last_metric: Instant,
    last_delta_time: f32,
    last_cam_pos: Point3<f32>,
    metric_interval: Duration,

    crosshair: Texture2D
}

impl InGameGUI {
    const METRIC_INTERVAL_SEC: f32 = 0.05;
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, ) -> Self {
        let metric_style = TextStyle {
            font: "Arial".to_owned(),
            color: Vector4::new(1., 1., 1., 1.),
            scale: 2.,
            affected_by_camera: false
        };

        let crosshair = Texture2D::from_png("Crosshair", device, queue,
         include_bytes!("../../resources/textures/crosshair.png"), wgpu::FilterMode::Nearest);

        Self {
            metric_style,
            last_metric: Instant::now(),
            last_delta_time: 0.,
            last_cam_pos: Point3::new(0., 0., 0.),
            metric_interval: Duration::from_secs_f32(Self::METRIC_INTERVAL_SEC),
            crosshair
        }
    }
}

impl Graphics {
    pub fn render_metrics(&mut self, renderables: &mut Renderables) {
        // Update Metrics
        if self.last_frame - self.gui.last_metric > self.gui.metric_interval {
            self.gui.last_metric = self.last_frame;
            self.gui.last_cam_pos = renderables.cam_pos;
            self.gui.last_delta_time = self.delta_time;
        }

        // Send metrics
        // FPS Counter
        renderables.sentences.push(Sentence {
            data: format!("{:.0} FPS", 1. / self.gui.last_delta_time),
            position: Vector3::new(-1.0, 0.9, 0.1),
            direction: Quaternion::new(1., 0., 0., 0.),
            text_style: self.gui.metric_style.clone(),
        });

        // Player position
        renderables.sentences.push(Sentence {
            data: format!("x: {:.2} y: {:.2} z: {:.2}",
                self.gui.last_cam_pos.x, self.gui.last_cam_pos.y, self.gui.last_cam_pos.z),
            position: Vector3::new(-1.0, 0.8, 0.1),
            direction: Quaternion::new(1., 0., 0., 0.),
            text_style: self.gui.metric_style.clone()
        });
    }

    pub fn render_crosshair(&self, render_pass: &mut wgpu::RenderPass) {

    }
}