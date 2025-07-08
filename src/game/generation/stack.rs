use std::collections::{ HashMap };

use super::slice::Slice;
use super::active_block::ActiveBlock;
use super::units::{ BlockLoc, Loc, BlockID };

pub struct Stack {
    slices: HashMap<Loc, Slice>,
    active_blocks: HashMap<BlockLoc, ActiveBlock>
}

impl Stack {
    const MAX_HEIGHT: Loc = 64;

    pub fn new() -> Self {
        let slices = HashMap::new();
        let active_blocks = HashMap::new();

        Self {
            slices,
            active_blocks
        }
    }

    pub fn get(&self, loc: BlockLoc) -> BlockID {
        match self.slices.get(&loc.y) {
            Some(slice) => slice.get(loc),
            None => 0
        }
    }
}