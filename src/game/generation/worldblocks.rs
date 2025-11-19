use std::collections::HashMap;
use cgmath::{ ElementWise, Vector3 };

use crate::util::range3d;
use crate::game::units::{BlockCoords, WorldCoords, BlockID};
use crate::graphics::cube_render::cube_instance::CubeInstance;
use super::stack::Stack;
use super::slice::Slice;
use super::super::units::{ StackCoords, EntityCoords, to_block_coord };
use super::super::components::{ collision::BoxCollider, spatial::Position };

pub struct WorldBlocks {
    stacks: HashMap<StackCoords, Stack>
}

impl WorldBlocks {
    pub const TOUCH_TOLERANCE: f32 = 0.2;
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

    // Returns the stack coordinates, the block position within the stack, and the stack itself
    pub fn get_stack_at(&self, position: BlockCoords) -> Option<(StackCoords, BlockCoords, &Stack)> {
        let coords = StackCoords {
            x: (position.x / Slice::X_SIZE),
            z: (position.z / Slice::Z_SIZE),
        };
        let offset = BlockCoords {
            x: position.x as i32 % Slice::X_SIZE,
            y: position.y as i32,
            z: position.z as i32 % Slice::Z_SIZE,
        };
        let stack = self.stacks.get(&coords);
        match stack {
            Some(stack) => return Some((coords, offset, stack)),
            None => None,
        }
    }

    pub fn get_stack_at_mut(&mut self, position: BlockCoords) -> Option<(StackCoords, BlockCoords, &mut Stack)> {
        let coords = StackCoords {
            x: (position.x / Slice::X_SIZE),
            z: (position.z / Slice::Z_SIZE),
        };
        let offset = BlockCoords {
            x: position.x as i32 % Slice::X_SIZE,
            y: position.y as i32,
            z: position.z as i32 % Slice::Z_SIZE,
        };
        let stack = self.stacks.get_mut(&coords);
        match stack {
            Some(stack) => return Some((coords, offset, stack)),
            None => None,
        }
    }

    pub fn get_block(&self, position: BlockCoords) -> Option<BlockID> {
        match self.get_stack_at(position) {
            Some((_, offset, stack)) => Some(stack.get_block(offset)),
            None => None
        }
    }

    pub fn set_block(&mut self, position: BlockCoords, id: BlockID) -> bool {
        match self.get_stack_at_mut(position) {
            Some((_, offset, stack)) => {
                stack.set_block(offset, id);
                true
            }
            None => false
        }
    }

    // Get a hashmap of coordinates and ids if a subset is small enough, for easier block checking
    pub fn get_subset(&self, position: WorldCoords, bounds: Vector3<f32>) -> HashMap<BlockCoords, BlockID> {
        // Determine which stacks
        let mut stacks: HashMap<StackCoords, &Stack> = HashMap::new();
        for x in [position.x - bounds.x, position.x + bounds.x] {
            for z in [position.z - bounds.z, position.z + bounds.z] {
                match self.get_stack_at(to_block_coord(WorldCoords { x, y: 0., z})) {
                    Some((coords, _, stack)) => { stacks.insert(coords, &stack); },
                    None => {},
                }
            }
        }

        let mut blocks: HashMap<BlockCoords, BlockID> = HashMap::new();
        for (coords, stack) in stacks {
            for y in (position.y - bounds.y) as i32..(position.y + bounds.y) as i32 + 1 {
                if let Some(slice) = stack.slices.get(&y) {
                    let coords = BlockCoords { x: coords.x * Slice::X_SIZE, y, z: coords.z * Slice::Z_SIZE };
                    slice.get_all_hash(&mut blocks, coords);
                }
            }
        };
        blocks
    }

    pub fn get_block_contact(&self, collider: &BoxCollider, position: &Position) -> Vec<(BlockID, Vector3<i32>, f32)> {
        let mut collisions = Vec::new();
        let blocks = self.get_subset(position.vector, collider.bounds); // Guarentees a possible position
        let (pos, bounds) = (position.vector, collider.bounds);
        let (corner_low, corner_high) = (pos - bounds, pos + bounds);
        let (corner_low_round, corner_high_round) = (corner_low.map(|c| c.round()), corner_high.map(|c| c.round()));
        let (corner_low_dis, corner_high_dis) = (
            corner_low.sub_element_wise(corner_low_round),
            corner_high.sub_element_wise(corner_high_round)
        );

        // Check if any face is touching a boundary
        // Collects all the blocks that could be touching that face and the direction vector
        let mut possibilities = Vec::new();
        if corner_low_dis.x.abs() < Self::TOUCH_TOLERANCE && corner_low_dis.x < 0.  {
            let blocks = range3d((corner_low.x.floor() as i32 - 1, corner_low.x.floor() as i32 + 1),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: -1, y: 0, z: 0}, corner_low.x - corner_low_round.x.round()));   
        }
        if corner_high_dis.x.abs() < Self::TOUCH_TOLERANCE && corner_high_dis.x < 0. {
            let blocks = range3d((corner_high.x.ceil() as i32 - 1, corner_high.x.ceil() as i32 + 1),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: 1, y: 0, z: 0}, corner_high.x - corner_high.x.round()));
        }
        if corner_low_dis.y.abs() < Self::TOUCH_TOLERANCE && corner_low_dis.y < 0. {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32),
                                 (corner_low.y.floor() as i32 - 1, corner_low.y.floor() as i32 + 1),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: 0, y: -1, z: 0}, corner_low.y - corner_low.y.round()));
        }
        if corner_high_dis.y.abs() < Self::TOUCH_TOLERANCE && corner_high_dis.y < 0. {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32),
                                 (corner_high.y.ceil() as i32 - 1, corner_high.y.ceil() as i32 + 1),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: 0, y: 1, z: 0}, corner_high.y - corner_high.y.round()));
        }
        if corner_low_dis.z.abs() < Self::TOUCH_TOLERANCE && corner_low_dis.z < 0. {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_low.z.floor() as i32 - 1, corner_low.z.floor() as i32 + 1));
            possibilities.push((blocks, Vector3 { x: 0, y: 0, z: -1}, corner_low.z - corner_low.z.round()));
        }
        if corner_high_dis.y.abs() < Self::TOUCH_TOLERANCE && corner_high_dis.y < 0. {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32 + 1),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_high.z.ceil() as i32 - 1, corner_high.z.ceil() as i32 + 1));
            possibilities.push((blocks, Vector3 { x: 0, y: 0, z: 1}, corner_high.z - corner_high.z.round()));
        }

        // Now check individual blocks
        // TODO: Replace with a map of blocks that can be walked through
        for (possible_blocks, direction, diff) in possibilities {
            for block in possible_blocks {
                match blocks.get(&block) { // Our subset guarentees a block location
                    Some(id_ref) => {
                        let id = *id_ref;
                        if id != 0 { // 0 is air block, can walk through it
                            collisions.push((id, direction, diff))
                        }
                    }
                    None => {}
                }
            }
        }
        collisions
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