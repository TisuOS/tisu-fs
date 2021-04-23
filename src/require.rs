use crate::{block_info::DiskInfo, leaf::Leaf, system::FileSystem};
use alloc::prelude::v1::*;

pub trait Format {
    fn to_system(&self)->FileSystem;
    fn parse_node(&self, block_idx : usize)->Result<Vec<Leaf>, ()>;
    fn get_block_chain(&self, start_idx : usize)->Result<Vec<usize>, ()>;
    fn parse_super_block(&self, block : &[u8])->DiskInfo;
}



