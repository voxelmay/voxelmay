use glam::IVec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const SHIFT: u32 = 4;
    pub const SIZE: i32 = 1 << Self::SHIFT;
    pub const MASK: i32 = Self::SIZE - 1;

    #[inline]
    pub fn from_world_pos(pos: IVec3) -> Self {
        Self {
            x: pos.x >> Self::SHIFT,
            y: pos.y >> Self::SHIFT,
            z: pos.z >> Self::SHIFT,
        }
    }

    /// Block position in chunk
    #[inline]
    pub fn local_pos(pos: IVec3) -> IVec3 {
        IVec3::new(pos.x & Self::MASK, pos.y & Self::MASK, pos.z & Self::MASK)
    }

    /// Minimal chunk corner in world pos
    #[inline]
    pub fn min_corner(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z) << Self::SHIFT
    }
}

#[test]
fn roundtrip() {
    for x in [-33, -17, -16, -1, 0, 15, 16, 47] {
        let w = IVec3::new(x, 7, -9);
        let c = ChunkPos::from_world_pos(w);
        let l = ChunkPos::local_pos(w);
        assert_eq!(c.min_corner() + l, w);
    }
}
