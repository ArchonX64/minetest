use crate::graphics::{cube_render::cube_instance::CubeInstance, text_render::sentence::Sentence};

use super::units::{ PlayerDirection, EntityCoords };

pub struct Renderables {
    pub cam_dir: PlayerDirection,
    pub cam_pos: EntityCoords,
    pub cubes: Vec<CubeInstance>,
    pub sentences: Vec<Sentence>
}