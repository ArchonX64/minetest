use cgmath::Vector3;
use legion::{ system, World, systems::Builder, IntoQuery };

use super::super::generation::worldblocks::WorldBlocks;

use super::spatial::{ Position, Velocity };

pub struct BoxCollider {
    pub bounds: Vector3<f32>
}

pub struct CollidesWithBlocks;

pub fn block_collide(world: &mut World, blocks: &WorldBlocks) {
    let mut query = <(&BoxCollider, &CollidesWithBlocks, &mut Position, &mut Velocity)>::query();

    for (collider, _, pos, vel) in query.iter_mut(world) {
        for (_id, dir, diff) in blocks.get_block_contact(collider, pos) {
            for (pos, vel, dir) in [(&mut pos.vector.x, &mut vel.vector.x, dir.x),
                              (&mut pos.vector.y, &mut vel.vector.y, dir.y),
                              (&mut pos.vector.z, &mut vel.vector.z, dir.z)] {
                if *vel != 0. && vel.signum() as i32 == dir.signum() {
                    *vel = 0.;
                    *pos += diff * dir.signum() as f32;
                }
            }
        }
    }
}