use std::collections::HashMap;

use cgmath::{Point3, Vector2};

use crate::graphics::cube_render::cube_instance::CubeInstance;
use super::super::units::{BlockCoords, BlockID};

pub struct Slice {
    blocks: Box<[BlockID; Self::SIZE]>
}

impl Slice {
    pub const X_SIZE: i32  = 16;
    pub const Z_SIZE: i32 = 16;
    pub const SIZE: usize = (Self::X_SIZE * Self::Z_SIZE) as usize;

    pub fn new(id: BlockID) -> Self {
        let blocks = Box::new([id; Self::SIZE]);

        Self {
            blocks
        }
    }

    // Vector2 represents stack on its own
    // Increases x, then z
    // Every z contains Self::XSIZE number of x values
    pub fn get(&self, position: BlockCoords) -> BlockID {
        self.blocks[Self::coords_to_array_pos(position)]
    }

    pub fn set_block(&mut self, position: BlockCoords, id: BlockID) {
        self.blocks[Self::coords_to_array_pos(position)] = id;
    }

    pub fn get_all_hash(&self, map: &mut HashMap<BlockCoords, BlockID>, offset: BlockCoords) {
        for (i, block) in (0i32..).zip(self.blocks.iter().copied()) {
            map.insert(Point3 {
                x: (offset.x + (i % Self::X_SIZE)),
                y: offset.y,
                z: (offset.z + (i / Self::X_SIZE)) 
            }, block);
        }
    }

    pub fn get_all(&self, storage: &mut Vec<CubeInstance>, offset: BlockCoords) {
        for (i, block) in (0i32..).zip(self.blocks.iter().copied()) {
            if block == 0 { continue } // Just air, no need for rendering

            storage.push(CubeInstance{ tex_index: (block as u32) - 1, position: Point3 {
                x: (offset.x + (i % Self::X_SIZE)) as f32,
                y: offset.y as f32,
                z: (offset.z + (i / Self::X_SIZE)) as f32 
            }});
        }
    }

    pub fn coords_to_array_pos(pos: BlockCoords) -> usize {
        return (pos.x + pos.z * Self::X_SIZE) as usize;
    }
}