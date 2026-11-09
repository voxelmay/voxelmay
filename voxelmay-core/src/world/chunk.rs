use glam::IVec3;

use crate::{block::block_id::BlockId, world::chunk_pos::ChunkPos};

pub const CHUNK_SIZE: usize = ChunkPos::SIZE as usize;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    blocks: Box<[BlockId; CHUNK_VOLUME]>,
    dirty: bool,
}

impl Chunk {
    pub fn new(fill: BlockId) -> Self {
        Self {
            blocks: Box::new([fill; CHUNK_VOLUME]),
            dirty: true,
        }
    }

    #[inline]
    fn index(local: IVec3) -> usize {
        debug_assert_eq!(ChunkPos::local_pos(local), local);
        let (x, y, z) = (local.x as usize, local.y as usize, local.z as usize);
        (y << ChunkPos::SHIFT | z) << ChunkPos::SHIFT | x
    }

    #[inline]
    pub fn get_local(&self, local: IVec3) -> BlockId {
        self.blocks[Self::index(local)]
    }

    #[inline]
    pub fn set_local(&mut self, local: IVec3, block: BlockId) -> bool {
        let i = Self::index(local);
        if self.blocks[i] == block {
            return false;
        }
        self.blocks[i] = block;
        self.dirty = true;
        true
    }

    #[inline]
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[inline]
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}
