#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlockId(pub u16);

impl BlockId {
    pub const AIR: BlockId = BlockId(0);
}
