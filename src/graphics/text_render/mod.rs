use rusttype::{point, Font, Scale};

struct TextRenderer {
    shader: wgpu::ShaderModule,
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl TextRenderer {
    pub fn new() {
        let fontbytes = include_bytes!("../../../resources/fonts/arial.ttf");

        let font = rusttype::Font::try_from_bytes(fontbytes).unwrap();
    }
}