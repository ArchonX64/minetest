use super::OPENGL_TO_WGPU_MATRIX;
use super::camera::CameraInitials;


pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(initials: &CameraInitials) -> Self {
        Self {
            aspect: initials.width / initials.height,
            fovy: initials.fovy,
            znear: initials.znear,
            zfar: initials.zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        return OPENGL_TO_WGPU_MATRIX * cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar)
    }
}