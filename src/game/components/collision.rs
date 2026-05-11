use cgmath::{Vector3, Zero};
use legion::{ World, IntoQuery, Resources };

use super::super::generation::worldblocks::WorldBlocks;

use super::spatial::{ Position, Velocity };

pub struct BoxCollider {
    pub bounds: Vector3<f32>
}

// Only valid after collisions have been performed
// Keeps track of collisions of the current frame
pub struct CollidesWithBlocks {
    pub collisions: Vector3<i32>
}

impl CollidesWithBlocks {
    pub fn new() -> Self {
        CollidesWithBlocks { collisions: Vector3::zero() }
    }
}

pub fn block_collide(world: &mut World, resources: &Resources) {
    let mut query = <(&BoxCollider, &mut CollidesWithBlocks, &mut Position, &mut Velocity)>::query();
    let blocks = resources.get::<WorldBlocks>().unwrap();

    for (collider, cwb, pos_vec, vel) in query.iter_mut(world) {
        cwb.collisions = Vector3::zero();

        for (_id, dir_vec, diff) in blocks.get_block_contact(collider, pos_vec) {
            let mut counter = 0;
            for (vel, dir) in [(&mut vel.vector.x, dir_vec.x), (&mut vel.vector.y, dir_vec.y), (&mut vel.vector.z, dir_vec.z)] {
                if *vel != 0. && vel.signum() as i32 == dir.signum() {
                    *vel = 0.;
                    cwb.collisions += dir_vec;

                    println!("BEGIN COLLISION");
                    println!("Position: {:.?}", pos_vec.vector);
                    println!("Direction: {} {}", counter, dir);
                    println!("Collider Upper: {:.?}", pos_vec.vector + collider.bounds);
                    println!("Collider Lower: {:.?}", pos_vec.vector - collider.bounds);
                }
                counter += 1;
            }
        }
    }
}