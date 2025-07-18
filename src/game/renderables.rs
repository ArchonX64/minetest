use crate::graphics::cube_render::cube_instance::CubeInstance;

use super::units::{ PlayerDirection };

pub struct Renderables {
    pub cam_dir: PlayerDirection,
    pub cubes: Vec<CubeInstance>
}