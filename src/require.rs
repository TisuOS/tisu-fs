use crate::{Directory, File, FileFlag, block_info::DiskInfo, leaf::Leaf, system::{FileSystem, IoError, IoResult}};
use alloc::prelude::v1::*;

pub trait Format {
    fn parse_node(&self, block_idx : usize)->Result<Vec<Leaf>, ()>;
    fn get_block_chain(&self, start_idx : usize)->Result<Vec<usize>, ()>;
    fn parse_super_block(&self)->DiskInfo;
    fn get_device(&self)->usize;
}

pub trait SystemOp {
    fn open(&mut self, path : String, flag : FileFlag)->Result<&File, ()>;

    /// 仅取得目录信息
    fn enter(&mut self, path : String)->Result<Directory, ()>;

    /// 取得文件信息
    fn get_file(&mut self, path : String)->Result<File, IoError>;

    fn read(&mut self, id : usize, data : &mut [u8])->IoResult;

    fn write(&mut self, id : usize, data : &[u8])->IoResult;

    fn total_size(&self)->usize;

    fn block_size(&self)->usize;

    /// 判断此文件系统是否包含该文件 ID
    fn contain(&self, id : usize)->bool;

    /// 刷新对应目录下的信息
    fn refresh(&mut self, dir : &Directory);

    fn check(&self)->usize;
}

