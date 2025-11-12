use cgmath::{ Quaternion, Point3, Vector3, Vector4, Zero };
use std::time::{ Duration, Instant };

use super::{ Graphics, Renderables, text_render::{ sentence::Sentence, text_style::TextStyle }};

pub struct PersistentMetrics {
    metric_style: TextStyle,
    last_metric: Instant,
    last_delta_time: f32,
    last_cam_pos: Point3<f32>,
    metric_interval: Duration,
}

impl PersistentMetrics {
    const METRIC_INTERVAL_SEC: f32 = 0.1;

    pub fn new() -> Self {
        let metric_style = TextStyle {
            font: "Arial".to_owned(),
            color: Vector4::new(1., 1., 1., 1.),
            scale: 2.,
            affected_by_camera: false
        };

        Self {
            metric_style,
            last_metric: Instant::now(),
            last_delta_time: 0.,
            last_cam_pos: Point3::new(0., 0., 0.),
            metric_interval: Duration::from_secs_f32(Self::METRIC_INTERVAL_SEC),
        }
    }
}

impl Graphics {
    pub fn metrics(&mut self, renderables: &mut Renderables) {
        // Update Metrics
        if self.last_frame - self.metrics.last_metric > self.metrics.metric_interval {
            self.metrics.last_metric = self.last_frame;
            self.metrics.last_cam_pos = renderables.cam_pos;
            self.metrics.last_delta_time = self.delta_time;
        }

        // Send metrics
        // FPS Counter
        renderables.sentences.push(Sentence {
            data: format!("{:.0} FPS", 1. / self.metrics.last_delta_time),
            position: Vector3::new(-1.0, 0.9, 0.1),
            direction: Quaternion::new(1., 0., 0., 0.),
            text_style: self.metrics.metric_style.clone(),
        });

        // Player position
        renderables.sentences.push(Sentence {
            data: format!("x: {:.1} y: {:.1} z: {:.1}",
                self.metrics.last_cam_pos.x, self.metrics.last_cam_pos.y, self.metrics.last_cam_pos.z),
            position: Vector3::new(-1.0, 0.8, 0.1),
            direction: Quaternion::new(1., 0., 0., 0.),
            text_style: self.metrics.metric_style.clone()
        });
    }
}