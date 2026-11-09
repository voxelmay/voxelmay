use crate::block::block_id::BlockId;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    blocks: Box<[BlockId; CHUNK_VOLUME]>,
    dirty: bool,
}
