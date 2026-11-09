use std::collections::HashMap;

use glam::IVec3;

use crate::{
    block::block_id::BlockId,
    world::{chunk::Chunk, chunk_pos::ChunkPos},
};

#[derive(Default)]
pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    pub(crate) fn insert_chunk(&mut self, pos: ChunkPos, chunk: Chunk) -> bool {
        if self.chunks.contains_key(&pos) {
            return false;
        }
        self.chunks.insert(pos, chunk);
        true
    }

    pub(crate) fn remove_chunk(&mut self, pos: ChunkPos) -> Option<Chunk> {
        self.chunks.remove(&pos)
    }

    pub(crate) fn chunk_positions(&self) -> impl Iterator<Item = ChunkPos> + '_ {
        self.chunks.keys().copied()
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    pub fn get_block(&self, pos: IVec3) -> Option<BlockId> {
        let chunk = self.chunks.get(&ChunkPos::from_world_pos(pos))?;
        Some(chunk.get_local(ChunkPos::local_pos(pos)))
    }

    pub fn set_block(&mut self, pos: IVec3, block: BlockId) {
        let cp = ChunkPos::from_world_pos(pos);
        let local = ChunkPos::local_pos(pos);
        let changed = self
            .chunks
            .entry(cp)
            .or_insert_with(|| Chunk::new(BlockId::AIR))
            .set_local(local, block);

        if changed {
            self.mark_boundary_neighbors_dirty(cp, local);
        }
    }

    fn mark_boundary_neighbors_dirty(&mut self, pos: ChunkPos, local: IVec3) {
        let mut neighbors = [None; 6];
        let mut count = 0;

        if local.x == 0 {
            neighbors[count] = Some(ChunkPos {
                x: pos.x - 1,
                ..pos
            });
            count += 1;
        }
        if local.x == ChunkPos::MASK {
            neighbors[count] = Some(ChunkPos {
                x: pos.x + 1,
                ..pos
            });
            count += 1;
        }
        if local.y == 0 {
            neighbors[count] = Some(ChunkPos {
                y: pos.y - 1,
                ..pos
            });
            count += 1;
        }
        if local.y == ChunkPos::MASK {
            neighbors[count] = Some(ChunkPos {
                y: pos.y + 1,
                ..pos
            });
            count += 1;
        }
        if local.z == 0 {
            neighbors[count] = Some(ChunkPos {
                z: pos.z - 1,
                ..pos
            });
            count += 1;
        }
        if local.z == ChunkPos::MASK {
            neighbors[count] = Some(ChunkPos {
                z: pos.z + 1,
                ..pos
            });
            count += 1;
        }

        for neighbor in neighbors[..count].iter().flatten() {
            if let Some(chunk) = self.chunks.get_mut(neighbor) {
                chunk.mark_dirty();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_change_marks_loaded_neighbor_dirty() {
        let mut world = World::default();
        world.set_block(IVec3::new(15, 0, 0), BlockId(1));
        world.set_block(IVec3::new(16, 0, 0), BlockId(1));

        world
            .chunks
            .get_mut(&ChunkPos { x: 0, y: 0, z: 0 })
            .unwrap()
            .clear_dirty();
        world
            .chunks
            .get_mut(&ChunkPos { x: 1, y: 0, z: 0 })
            .unwrap()
            .clear_dirty();

        world.set_block(IVec3::new(15, 0, 0), BlockId(2));

        assert!(
            world
                .get_chunk(ChunkPos { x: 0, y: 0, z: 0 })
                .unwrap()
                .is_dirty()
        );
        assert!(
            world
                .get_chunk(ChunkPos { x: 1, y: 0, z: 0 })
                .unwrap()
                .is_dirty()
        );
    }

    #[test]
    fn unchanged_boundary_block_does_not_dirty_neighbor() {
        let mut world = World::default();
        world.set_block(IVec3::new(15, 0, 0), BlockId(1));
        world.set_block(IVec3::new(16, 0, 0), BlockId(1));

        world
            .chunks
            .get_mut(&ChunkPos { x: 0, y: 0, z: 0 })
            .unwrap()
            .clear_dirty();
        world
            .chunks
            .get_mut(&ChunkPos { x: 1, y: 0, z: 0 })
            .unwrap()
            .clear_dirty();

        world.set_block(IVec3::new(15, 0, 0), BlockId(1));

        assert!(
            !world
                .get_chunk(ChunkPos { x: 1, y: 0, z: 0 })
                .unwrap()
                .is_dirty()
        );
    }
}
