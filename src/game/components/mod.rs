pub mod spatial;
pub mod time;
pub mod input;
pub mod collision;

use spatial::*;
use collision::*;

pub struct TotalPhysics {
    position: Position,
    velocity: Velocity,
    collider: BoxCollider,
    bc: CollidesWithBlocks, 
}