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

    pub fn get(&mut self)->usize {
        if let Some(id) = self.used.pop() { id }
        else { self.id.add() }
    }
}