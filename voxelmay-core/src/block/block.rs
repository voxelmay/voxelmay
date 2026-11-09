use crate::block::block_id::BlockId;

pub struct Block {
    pub id: BlockId,
    pub solid: bool,
    pub transparent: bool,
}
