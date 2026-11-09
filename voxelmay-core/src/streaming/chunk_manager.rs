use crate::{
    generation::{TerrainGenerator, generator::Generator},
    world::{chunk_pos::ChunkPos, world::World},
};

pub struct ChunkManager {
    load_radius: i32, // in chunks
    unload_radius: i32,
    per_frame_budget: usize,
    generator: Box<dyn Generator>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkManagerConfigError {
    NegativeLoadRadius,
}

impl ChunkManager {
    pub fn new(
        load_radius: i32,
        per_frame_budget: usize,
        seed: u64,
    ) -> Result<Self, ChunkManagerConfigError> {
        Self::with_generator(load_radius, per_frame_budget, TerrainGenerator::new(seed))
    }

    pub fn with_generator<G: Generator + 'static>(
        load_radius: i32,
        per_frame_budget: usize,
        generator: G,
    ) -> Result<Self, ChunkManagerConfigError> {
        if load_radius < 0 {
            return Err(ChunkManagerConfigError::NegativeLoadRadius);
        }

        Ok(Self {
            load_radius,
            unload_radius: load_radius.saturating_add(2),
            per_frame_budget,
            generator: Box::new(generator),
        })
    }

    pub fn update(&mut self, world: &mut World, center: ChunkPos) {
        let unload_radius_squared = i128::from(self.unload_radius).pow(2);
        let chunks_to_unload: Vec<_> = world
            .chunk_positions()
            .filter(|pos| distance_squared(*pos, center) > unload_radius_squared)
            .collect();
        for pos in chunks_to_unload {
            world.remove_chunk(pos);
        }

        if self.per_frame_budget == 0 {
            return;
        }

        let load_radius_squared = i128::from(self.load_radius).pow(2);
        let radius = i64::from(self.load_radius);
        let mut missing = Vec::new();

        for y in -radius..=radius {
            for z in -radius..=radius {
                for x in -radius..=radius {
                    let distance = i128::from(x) * i128::from(x)
                        + i128::from(y) * i128::from(y)
                        + i128::from(z) * i128::from(z);
                    if distance > load_radius_squared {
                        continue;
                    }

                    let Some(pos) = offset(center, x, y, z) else {
                        continue;
                    };
                    if !is_world_chunk(pos) {
                        continue;
                    }
                    if world.get_chunk(pos).is_none() {
                        missing.push((distance, pos));
                    }
                }
            }
        }

        missing.sort_unstable_by_key(|(distance, pos)| (*distance, pos.x, pos.y, pos.z));

        for (_, pos) in missing.into_iter().take(self.per_frame_budget) {
            let chunk = self.generator.generate(pos);
            world.insert_chunk(pos, chunk);
        }
    }
}

fn distance_squared(a: ChunkPos, b: ChunkPos) -> i128 {
    let dx = i128::from(a.x) - i128::from(b.x);
    let dy = i128::from(a.y) - i128::from(b.y);
    let dz = i128::from(a.z) - i128::from(b.z);
    dx * dx + dy * dy + dz * dz
}

fn offset(center: ChunkPos, x: i64, y: i64, z: i64) -> Option<ChunkPos> {
    Some(ChunkPos {
        x: i32::try_from(i64::from(center.x) + x).ok()?,
        y: i32::try_from(i64::from(center.y) + y).ok()?,
        z: i32::try_from(i64::from(center.z) + z).ok()?,
    })
}

fn is_world_chunk(pos: ChunkPos) -> bool {
    const MIN: i32 = i32::MIN >> ChunkPos::SHIFT;
    const MAX: i32 = i32::MAX >> ChunkPos::SHIFT;
    (MIN..=MAX).contains(&pos.x) && (MIN..=MAX).contains(&pos.y) && (MIN..=MAX).contains(&pos.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::chunk::Chunk;

    #[test]
    fn update_generates_nearest_chunks_within_budget() {
        let mut world = World::default();
        let mut manager = ChunkManager::new(2, 2, 123).unwrap();
        let center = ChunkPos { x: 0, y: 0, z: 0 };

        manager.update(&mut world, center);

        assert!(world.get_chunk(center).is_some());
        assert_eq!(
            world
                .chunk_positions()
                .filter(|pos| distance_squared(*pos, center) <= 4)
                .count(),
            2
        );
    }

    #[test]
    fn update_unloads_chunks_beyond_hysteresis_radius() {
        let mut world = World::default();
        let mut manager = ChunkManager::new(0, 0, 123).unwrap();
        let old = ChunkPos { x: 0, y: 0, z: 0 };
        let center = ChunkPos { x: 3, y: 0, z: 0 };
        world.insert_chunk(old, Chunk::new(crate::block::block_id::BlockId::AIR));

        manager.update(&mut world, center);

        assert!(world.get_chunk(old).is_none());
    }

    #[test]
    fn negative_load_radius_is_rejected() {
        assert_eq!(
            ChunkManager::new(-1, 1, 0).err(),
            Some(ChunkManagerConfigError::NegativeLoadRadius)
        );
    }
}
