use cgmath::Vector4;

#[derive(Clone, Debug)]
pub struct TextStyle {
    pub font: String,
    pub color: Vector4<f32>,
    pub scale: f32,
    pub affected_by_camera: bool
}