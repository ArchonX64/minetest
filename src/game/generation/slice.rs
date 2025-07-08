use super::units::{BlockLoc, Loc, BlockID};

pub struct Slice {
    blocks: Box<[BlockID; Self::SIZE]>
}

impl Slice {
    const X_SIZE: Loc  = 16;
    const Z_SIZE: Loc = 16;
    const SIZE: usize = (Self::X_SIZE * Self::Z_SIZE) as usize;

    pub fn new() -> Self {
        let blocks = Box::new([0; Self::SIZE]);

        Self {
            blocks
        }
    }

    pub fn get(&self, loc: BlockLoc) -> BlockID {
        self.blocks[(loc.x + loc.z * Self::X_SIZE) as usize]
    }
}