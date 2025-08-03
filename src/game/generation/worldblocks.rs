use std::collections::HashMap;
use cgmath::{ Point3, Vector3 };

use crate::util::range3d;
use crate::game::units::{BlockCoords, BlockID};
use crate::graphics::cube_render::cube_instance::CubeInstance;
use super::stack::Stack;
use super::slice::Slice;
use super::super::units::{ StackCoords, EntityCoords };
use super::super::components::{ collision::BoxCollider, spatial::Position };

pub struct WorldBlocks {
    stacks: HashMap<StackCoords, Stack>
}

impl WorldBlocks {
    pub const TOUCH_TOLERANCE: f32 = 0.1;
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

    // Get a hashmap of coordinates and ids if a subset is small enough, for easier block checking
    pub fn get_subset(& self, position: Point3<f32>, bounds: Vector3<f32>) -> HashMap<BlockCoords, BlockID> {
        // Determine which stacks
        let mut stacks: HashMap<StackCoords, &Stack> = HashMap::new();
        for x in [position.x - bounds.x, position.x + bounds.x] {
            for z in [position.z - bounds.z, position.z + bounds.z] {
                let coords = Stack::to_stack_coords(&Point3 { x, y: 0., z});

                match self.stacks.get(&coords) {
                    Some(stack) => {
                        if !stacks.contains_key(&coords) {
                            stacks.insert(coords, &stack);
                        }
                    },
                    None => {}
                }
            }
        }

        let mut blocks: HashMap<BlockCoords, BlockID> = HashMap::new();
        for (coords, stack) in stacks {
            for y in (position.y - bounds.y) as i32..(position.y + bounds.y) as i32 + 1 {
                if let Some(slice) = stack.slices.get(&y) {
                    let coords = BlockCoords { x: coords.x, y, z: coords.z };
                    slice.get_all_hash(&mut blocks, coords);
                }
            }
        };
        blocks
    }

    pub fn is_touching(&self, collider: &BoxCollider, position: &Position) -> Vec<(BlockID, Vector3<i32>)> {
        let mut collisions = Vec::new();
        let blocks = self.get_subset(position.vector, collider.bounds); // Guarentees a possible position
        let (pos, bounds) = (position.vector, collider.bounds);
        let (corner_low, corner_high) = (pos - bounds, pos + bounds);
        let (corner_low_round, corner_high_round) = (corner_low.map(|x| x.round()), corner_high.map(|x| x.round()));

        // Check if any face is touching a boundary
        // Collects all the blocks that could be touching that face and the direction vector
        let mut possibilities = Vec::new();
        if (corner_low.x - corner_low_round.x).abs() < Self::TOUCH_TOLERANCE {
            let blocks = range3d((corner_low.x.floor() as i32, corner_low.x.floor() as i32 + 1),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: -1, y: 0, z: 0}));   
        }
        if (corner_high.x - corner_high.x.round()).abs() < Self::TOUCH_TOLERANCE {
            let blocks = range3d((corner_high.x.ceil() as i32, corner_high.x.ceil() as i32 + 1),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: 1, y: 0, z: 0}));
        }
        if (corner_low.y - corner_low.y.round()).abs() < Self::TOUCH_TOLERANCE {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32),
                                 (corner_low.y.floor() as i32, corner_low.y.floor() as i32 + 1),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: 0, y: -1, z: 0}));
            println!("{:.?}", possibilities);
        }
        if (corner_high.y - corner_high.y.round()).abs() < Self::TOUCH_TOLERANCE {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32),
                                 (corner_high.y.ceil() as i32, corner_high.y.ceil() as i32 + 1),
                                 (corner_low_round.z as i32, corner_high_round.z as i32));
            possibilities.push((blocks, Vector3 { x: 0, y: 1, z: 0}));
        }
        if (corner_low.z - corner_low.z.round()).abs() < Self::TOUCH_TOLERANCE {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_low.z.floor() as i32, corner_low.z.floor() as i32 + 1));
            possibilities.push((blocks, Vector3 { x: 0, y: 0, z: -1}));
        }
        if (corner_high.z - corner_high.z.round()).abs() < Self::TOUCH_TOLERANCE {
            let blocks = range3d((corner_low_round.x as i32, corner_high_round.x as i32 + 1),
                                 (corner_low_round.y as i32, corner_high_round.y as i32),
                                 (corner_high.z.ceil() as i32, corner_high.z.ceil() as i32 + 1));
            possibilities.push((blocks, Vector3 { x: 0, y: 0, z: 1}));
        }

        // Now check individual blocks
        // TODO: Replace with a map of blocks that can be walked through
        for (possible_blocks, direction) in possibilities {
            for block in possible_blocks {
                match blocks.get(&block) { // Our subset guarentees a block location
                    Some(id_ref) => {
                        let id = *id_ref;
                        if id != 0 { // 0 is air block, can walk through it
                            collisions.push((id, direction))
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