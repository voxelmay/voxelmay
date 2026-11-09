use std::collections::HashMap;

use crate::world::{chunk::Chunk, chunk_pos::ChunkPos};

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    pub fn get_block() {
        unimplemented!()
    }

    pub fn set_block() {
        unimplemented!()
    }
}
