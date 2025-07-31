use crate::graphics::cube_render::cube_instance::CubeInstance;

use super::units::{ PlayerDirection, EntityCoords };

pub struct Renderables {
    pub cam_dir: PlayerDirection,
    pub cam_pos: EntityCoords,
    pub cubes: Vec<CubeInstance>
}