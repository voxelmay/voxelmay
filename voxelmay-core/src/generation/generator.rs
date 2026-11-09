use crate::world::{chunk::Chunk, chunk_pos::ChunkPos};

pub trait Generator {
    fn generate(&self, chunk_pos: ChunkPos) -> Chunk;
}
