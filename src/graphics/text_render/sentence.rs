use super::text_style::TextStyle;

use cgmath::{ Vector3, Quaternion };

pub struct Sentence {
    pub data: String,
    pub position: Vector3<f32>,
    pub direction: Quaternion<f32>,
    pub text_style: TextStyle
}