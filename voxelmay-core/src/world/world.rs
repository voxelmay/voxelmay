use std::collections::HashMap;

use crate::world::{chunk::Chunk, chunk_pos::ChunkPos};

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
}