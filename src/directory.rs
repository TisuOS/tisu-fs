/// ## 目录结构
/// 作为目录信息的体现，不具备交互功能
pub struct Directory{
    pub name : String,
    pub block_idx : usize,
    pub device_id : usize,
    pub path : String,
    pub item : Vec<DirectoryItem>
}

impl Directory {
}

pub struct DirectoryItem {
    pub name : String,
    pub itype : DirItemType,
}

impl DirectoryItem {
    pub fn is_file(&self)->bool {
        self.itype == DirItemType::File
    }

    pub fn is_dir(&self)->bool {
        self.itype == DirItemType::Directory
    }
}

#[derive(PartialEq)]
pub enum DirItemType {
    Directory,
    File,
}


use alloc::prelude::v1::*;
