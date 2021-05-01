use tisu_sync::AtomCounter;
use alloc::prelude::v1::*;

pub struct IdManager {
    id : AtomCounter,
    used : Vec<usize>,
}

impl IdManager {
    pub fn new()->Self {
        Self {
            id:AtomCounter::new(),
            used : Vec::new(),
        }
    }

    /// 预留 0、1 给标准输入输出
    pub fn get(&mut self)->usize {
        if let Some(id) = self.used.pop() { id }
        else {
            let mut rt = self.id.add();
            while rt <= 1 {
                rt = self.id.add();
            }
            rt
        }
    }
}