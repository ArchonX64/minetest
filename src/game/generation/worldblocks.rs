use std::collections::HashMap;

use crate::graphics::cube_render::cube_instance::CubeInstance;
use super::stack::Stack;
use super::slice::Slice;
use super::super::units::{ StackCoords, EntityCoords };

pub struct WorldBlocks {
    stacks: HashMap<StackCoords, Stack>
}

impl WorldBlocks {
    pub const STACK_RENDER_BOUND: i32 = 3;
    pub const BLOCK_RENDER_COUNT: i32 = Slice::X_SIZE * Slice::Z_SIZE * Stack::MAX_HEIGHT * Self::STACK_RENDER_BOUND * Self::STACK_RENDER_BOUND;

    pub fn test_layout() -> Self {
        let mut stacks = HashMap::new();

        for x in 0..3 {
            for z in 0..3 {
                let coords = StackCoords { x, z };
                stacks.insert(coords, Stack::test_layout());
            }
        };

        Self {
            stacks
        }
    }

    pub fn get_renderable_blocks(&mut self, position: EntityCoords) -> Vec<CubeInstance> {
        // Render the 3x3 chunk area around player
        let mut blocks = Vec::with_capacity(Self::BLOCK_RENDER_COUNT as usize);
        let stackcoords = Stack::to_stack_coords(&position);

        for x in (stackcoords.x - Self::STACK_RENDER_BOUND)..(stackcoords.x + Self::STACK_RENDER_BOUND) {
            for z in (stackcoords.z - Self::STACK_RENDER_BOUND)..(stackcoords.z + Self::STACK_RENDER_BOUND) {
                let coords = StackCoords { x, z };
                let stack = match self.stacks.get(&coords) {
                    Some(stack) => stack,
                    None => continue  // Don't render ungenerated chunks, later will trigger generation
                };

                stack.all_blocks(&mut blocks, coords);
            }
        };

        blocks
    }
}