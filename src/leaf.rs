use alloc::prelude::v1::*;

pub struct Leaf {
    pub name : String,
    pub ltype : LeafType,
    pub block_idx : usize,
    pub size : usize,
}

impl Leaf {
    pub fn is_file(&self)->bool {
        self.ltype == LeafType::File
    }

    pub fn is_directory(&self)->bool {
        self.ltype == LeafType::Directory
    }
}

impl Clone for Leaf {
    fn clone(&self) -> Self {
        Self {
            name : self.name.clone(),
            ltype : self.ltype,
            block_idx : self.block_idx,
            size : self.size,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeafType {
    File,
    Directory,
}

