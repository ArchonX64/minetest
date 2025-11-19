use cgmath::{ Vector3, Point2, Point3 };

pub type Loc = i32;
pub type BlockID = u8;
pub type WorldCoords = Point3<f32>;
pub type EntityCoords = Point3<f32>;
pub type BlockCoords = Point3<i32>;
pub type PlayerDirection = Vector3<f32>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct StackCoords {
    pub x: i32,
    pub z: i32,
}

pub fn to_block_coord(position: WorldCoords) -> BlockCoords{
    return position.map(|c| c.floor() as i32);
}