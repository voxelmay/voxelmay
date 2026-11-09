use std::collections::HashMap;

use crate::block::block_id::BlockId;

#[derive(Clone, Copy)]
pub struct BlockDef {
    pub name: &'static str,
    pub solid: bool,
    pub transparent: bool,
}

pub struct BlockRegistry {
    defs: Vec<BlockDef>,                 // BlockId.0
    ids: HashMap<&'static str, BlockId>, // name -> BlockId
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut r = Self {
            defs: Vec::new(),
            ids: HashMap::new(),
        };
        r.register(BlockDef {
            name: "air",
            solid: false,
            transparent: true,
        });
        r
    }

    pub fn register(&mut self, def: BlockDef) -> BlockId {
        let id = BlockId(self.defs.len() as u16);
        self.defs.push(def);
        self.ids.insert(def.name, id);
        id
    }

    #[inline]
    pub fn def(&self, id: BlockId) -> BlockDef {
        self.defs[id.0 as usize]
    }

    pub fn id_of(&self, name: &str) -> Option<BlockId> {
        self.ids.get(name).copied()
    }
}
