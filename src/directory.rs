/// ## 目录结构
/// 将磁盘文件、目录当作一个树形构造，一次提供一层（一个目录）的切片
/// 提供向上（前往父目录），向下（前往子目录）的功能，其它功能待补充
/// 父目录按照簇号存储进 Vec 中
pub struct Directory{
    pub name : String,
    pub block_idx : usize,
    pub path : String,
    pub item : Vec<DirectoryItem>
}

impl Directory {
}

pub struct DirectoryItem {
    pub name : String,
    pub itype : DirItemType,
}

pub enum DirItemType {
    Directory,
    File,
}


use alloc::prelude::v1::*;
