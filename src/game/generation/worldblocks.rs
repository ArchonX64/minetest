use core::f32;
use std::collections::HashMap;
use cgmath::{ ElementWise, EuclideanSpace, InnerSpace, MetricSpace, Point3, Vector3, Zero };

use crate::util::{ range3d, EPSILON };
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
    pub fn get_subset_from_center(&self, position: WorldCoords, bounds: Vector3<f32>) -> HashMap<BlockCoords, BlockID> {
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

    pub fn get_subset(&self, bound1: WorldCoords, bound2: WorldCoords) -> HashMap<BlockCoords, BlockID> {
        let mut stacks: HashMap<StackCoords, &Stack> = HashMap::new();
        for x in [bound1.x.min(bound2.x), bound1.x.max(bound2.x)] {
            for z in [bound1.z.min(bound2.z), bound1.x.max(bound2.z)] {
                match self.get_stack_at(to_block_coord(WorldCoords { x, y: 0., z})) {
                    Some((coords, _, stack)) => { stacks.insert(coords, &stack); },
                    None => {},
                }
            }
        }

        let mut blocks: HashMap<BlockCoords, BlockID> = HashMap::new();
        for (coords, stack) in stacks {
            for y in (bound1.y.min(bound2.y)) as i32..(bound1.y.max(bound2.y)) as i32 + 1 {
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
        let blocks = self.get_subset_from_center(position.vector, collider.bounds); // Guarentees a possible position
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

    // Get all blocks hit by a raycast
    // Returns a list of tuples containing block properties and the face
    pub fn get_raycast(&self, position: WorldCoords, length: f32, direction: Vector3<f32>) -> Vec<(BlockCoords, BlockID, Vector3<i32>)> {
        let dirvec = direction.normalize();
        let subset = self.get_subset(position, position + dirvec * length);

        let mut curpos = position.clone().to_vec();
        let mut curblock = to_block_coord(position).clone();

        let mut blocks = Vec::new();
        let mut distance = 0.;
        //println!("===== BEGIN RAYCAST =====");
        loop {
            // Calculate a factor that represents ability to get to a face quickest
            // Usually, the ray goes from a spot inside the cube towards the nearest edge.
            // Other times, we are already at the edge, and we must find the next edge to travel towards
            let deltas = curpos.zip(dirvec, |p, d| {
                if d.abs() < EPSILON {
                    f32::INFINITY
                } else if d > 0. {
                    (if (p - p.ceil()).abs() > EPSILON { p.ceil() - p } else { 1. } / d).abs()
                } else {
                    (if (p - p.floor()).abs() > EPSILON { p - p.floor() } else { -1. } / d).abs()
                }
            });

            // The one with the smallest delta will be the one traveled to reach the nearest face
            // Travel them all simultaneously, and then check which one hits a face
            let min_delta = deltas.x.min(deltas.y).min(deltas.z);
            curpos += dirvec * min_delta;
            distance += min_delta; // Somehow the min_delta is the distance idk i trust math

            // Determine if a face has been reached (one is guarenteed due to minimum check)
            let mut curface = Vector3::zero();
            if (deltas.x - min_delta).abs() < EPSILON {
                curblock.x += dirvec.x.signum() as i32;
                curface += Vector3::unit_x() * (-dirvec.x.signum() as i32);
            } if (deltas.y - min_delta).abs() < EPSILON {
                curblock.y += dirvec.y.signum() as i32;
                curface += Vector3::unit_y() * (-dirvec.y.signum() as i32);
            } 
            if (deltas.z - min_delta).abs() < EPSILON {
                curblock.z += dirvec.z.signum() as i32;
                curface += Vector3::unit_z() * (-dirvec.z.signum() as i32);
            }
            if curface.is_zero() {
                panic!("Block raycast failed due to all directional delta components being NAN, or a failure in the ordering promise.")
            }

            //println!("Dirvec: {:?}", dirvec);
            //println!("Deltas: {:?}", deltas);
            //println!("Curpos: {:?}", curpos);
            //println!("Curblock: {:?}", curblock);
            //println!("Distance: {:?}", position.distance(Point3::from_vec(curpos)));
            
            // Stop and return the raycast if the length has been reached
            if distance > length {
                return blocks
            }

            // Append block if length has not yet been reached, do not add if not generated yet
            if let Some(block) = subset.get(&curblock) {
                blocks.push((curblock.clone(), *block, curface.clone()));
            }
            
        };
    }

    //Get the first non-air block hit by a raycast
    pub fn get_raycast_intersect(&self, position: WorldCoords, length: f32, direction: Vector3<f32>) -> Option<(BlockCoords, BlockID, Vector3<i32>)> {
        for (loc, block, face) in self.get_raycast(position, length, direction) {
            if block != 0 {
                //println!("Block placement successful, found block at {:?}, placed block at {:?}", loc, loc + face);
                return Some((loc, block, face))
            }
        };
        //println!("Did not find a non-air block within range");
        None
    }

    pub fn get_renderable_blocks(&self, position: EntityCoords) -> Vec<CubeInstance> {
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