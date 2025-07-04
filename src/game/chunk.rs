use cgmath::Point3;

use super::block::Block;

pub struct Chunk {
    blocks: Box<Block>
}

impl Chunk {
    const CHUNKSIZE_X: u32 = 16;
    const CHUNKSIZE_Y: u32 = 128;
    const CHUNKSIZE_Z: u32 = 16;
    pub fn new() -> Self {
        let size = Self::CHUNKSIZE_X * Self::CHUNKSIZE_Y * Self::CHUNKSIZE_Z;
        let blocks = Box<[Block, size]>::new
    }

    // Indexing starts at 0, 0, 0 (bottom of chunk)
    // Increases x, z, y in order
    pub fn get(&self, loc: &Point3<u32>) -> Block {
        return self.blocks[loc.x + loc.z * ]
    }
}