use glam::IVec3;

use crate::{
    block::block_id::BlockId,
    generation::generator::Generator,
    world::{
        chunk::{CHUNK_SIZE, Chunk},
        chunk_pos::ChunkPos,
    },
};

const TERRAIN_SCALE: f64 = 0.025;

pub struct TerrainGenerator {
    seed: u64,
}

impl TerrainGenerator {
    pub const fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn generate(&self, pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(BlockId::AIR);
        let origin = pos.min_corner();

        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let world_x = origin.x + x as i32;
                let world_z = origin.z + z as i32;
                let height = self.height_at(world_x, world_z);

                for y in 0..CHUNK_SIZE {
                    let world_y = origin.y + y as i32;
                    let depth = i64::from(height) - i64::from(world_y);
                    let block = if depth == 0 {
                        BlockId::GRASS
                    } else if (1..=4).contains(&depth) {
                        BlockId::DIRT
                    } else if depth > 4 {
                        BlockId::STONE
                    } else {
                        continue;
                    };

                    chunk.set_local(IVec3::new(x as i32, y as i32, z as i32), block);
                }
            }
        }

        chunk
    }

    fn height_at(&self, x: i32, z: i32) -> i32 {
        let broad = self.sample(x as f64 * TERRAIN_SCALE, z as f64 * TERRAIN_SCALE);
        let detail = self.sample(
            x as f64 * TERRAIN_SCALE * 3.0,
            z as f64 * TERRAIN_SCALE * 3.0,
        );
        (broad * 20.0 + detail * 4.0 + 8.0).round() as i32
    }

    fn sample(&self, x: f64, z: f64) -> f64 {
        let x0 = x.floor() as i64;
        let z0 = z.floor() as i64;
        let tx = smooth(x - x0 as f64);
        let tz = smooth(z - z0 as f64);

        let a = self.lattice(x0, z0);
        let b = self.lattice(x0 + 1, z0);
        let c = self.lattice(x0, z0 + 1);
        let d = self.lattice(x0 + 1, z0 + 1);

        lerp(lerp(a, b, tx), lerp(c, d, tx), tz)
    }

    fn lattice(&self, x: i64, z: i64) -> f64 {
        let mut value = self.seed
            ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ (z as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^= value >> 31;
        (value as f64 / u64::MAX as f64) * 2.0 - 1.0
    }
}

impl Generator for TerrainGenerator {
    fn generate(&self, pos: ChunkPos) -> Chunk {
        TerrainGenerator::generate(self, pos)
    }
}

fn smooth(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic_for_same_seed() {
        let generator = TerrainGenerator::new(42);
        let first = generator.generate(ChunkPos { x: 0, y: 0, z: 0 });
        let second = generator.generate(ChunkPos { x: 0, y: 0, z: 0 });

        for y in 0..CHUNK_SIZE as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                for x in 0..CHUNK_SIZE as i32 {
                    let local = IVec3::new(x, y, z);
                    assert_eq!(first.get_local(local), second.get_local(local));
                }
            }
        }
    }
}
