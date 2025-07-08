
pub type Loc = u32;
pub type BlockID = u8;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BlockLoc {
    pub x: Loc,
    pub y: Loc,
    pub z: Loc
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct StackCoords {
    pub x: Loc,
    pub z: Loc,
}