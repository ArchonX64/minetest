use std::collections::{ HashMap };

use crate::graphics::cube_render::cube_instance::CubeInstance;
use super::slice::Slice;
use super::active_block::ActiveBlock;
use super::super::units::{ BlockLoc, Loc, StackCoords, PlayerCoords, BlockID };

pub struct Stack {
    slices: HashMap<Loc, Slice>,
    active_blocks: HashMap<BlockLoc, ActiveBlock>
}

impl Stack {
    pub const MAX_HEIGHT: Loc = 32;

    pub fn new() -> Self {
        let slices = HashMap::new();
        let active_blocks = HashMap::new();

        Self {
            slices,
            active_blocks
        }
    }

    pub fn test_layout() -> Self {
        let mut stack  = Stack::new();
        
        for i in 0u32..3u32 {
            stack.slices.insert(i, Slice::new(0));
        };

        stack
    }

    pub fn get(&self, loc: BlockLoc) -> BlockID {
        match self.slices.get(&loc.y) {
            Some(slice) => slice.get(loc),
            None => 0
        }
    }

    pub fn all_blocks(&self, storage: &mut Vec<CubeInstance>, coords: StackCoords) {
        let offset = Self::from_stack_coords(&coords);
        for (y, slice) in &self.slices {
            slice.get_all(storage, BlockLoc {
                x: offset.0,
                y: *y,
                z: offset.1
            });
        };
    }

    pub fn to_stack_coords(position: &PlayerCoords) -> StackCoords {
        StackCoords {
            x: (position.x / Slice::X_SIZE as f32).floor() as u32,
            z: (position.z / Slice::Z_SIZE as f32).floor() as u32
        }
    }

    pub fn from_stack_coords(position: &StackCoords) -> (u32, u32) {
        (position.x * Slice::X_SIZE, position.z * Slice::Z_SIZE)
    }
}