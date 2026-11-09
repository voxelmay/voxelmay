use std::collections::HashMap;

use glam::IVec3;

use crate::{
    block::block_id::BlockId,
    world::{
        chunk::{self, Chunk},
        chunk_pos::ChunkPos,
    },
};

#[derive(Default)]
pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    pub fn get_block(&self, pos: IVec3) -> Option<BlockId> {
        let chunk = self.chunks.get(&ChunkPos::from_world_pos(pos))?;
        Some(chunk.get_local(ChunkPos::local_pos(pos)))
    }

    pub fn set_block(&mut self, pos: IVec3, block: BlockId) {
        let cp = ChunkPos::from_world_pos(pos);
        let chunk = self
            .chunks
            .entry(cp)
            .or_insert_with(|| Chunk::new(BlockId::AIR));
        chunk.set_local(ChunkPos::local_pos(pos), block);
    }
}
