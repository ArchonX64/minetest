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
        for (_id, dir, diff) in blocks.is_touching(collider, pos) {
            for (p, v, d) in [(&mut pos.vector.x, &mut vel.vector.x, dir.x),
                              (&mut pos.vector.y, &mut vel.vector.y, dir.y),
                              (&mut pos.vector.z, &mut vel.vector.z, dir.z)] {
                if *v != 0. && v.signum() as i32 == d.signum() {
                    *v = 0.;
                    *p += diff * d.signum() as f32;
                }
            }
        }
    }
}