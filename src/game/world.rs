use std::collections::HashMap;

use cgmath::Point3

use super::chunk::Chunk;

struct World {
    chunks: HashMap<Point3<u32>, Chunk>
}