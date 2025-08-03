use cgmath::Vector3;
use legion::{ system, World, systems::Builder, IntoQuery };

use super::super::generation::worldblocks::WorldBlocks;

use super::spatial::{ Position, Velocity };

pub struct BoxCollider {
    pub bounds: Vector3<f32>
}

pub struct CollidesWithBlocks;


pub fn block_collide(world: &mut World, blocks: &WorldBlocks) {
    let mut query = <(&BoxCollider, &CollidesWithBlocks, &Position, &mut Velocity)>::query();

    for (collider, _, pos, vel) in query.iter_mut(world) {
        for (_id, dir) in blocks.is_touching(collider, pos) {
            println!("{:.?}", dir);
            
            for (v, d) in [(&mut vel.vector.x, dir.x), (&mut vel.vector.y, dir.y), (&mut vel.vector.z, dir.z)] {
                if *v != 0. && v.signum() as i32 == d.signum() {
                    *v = 0.;
                }
            }
        }
    }
}