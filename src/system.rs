use core::cmp::min;

use alloc::collections::BTreeMap;
use alloc::prelude::v1::*;
use device_buffer::CacheBuffer;
use crate::{DirectoryItem, FileFlag, Leaf, directory::Directory, file::{File, FileState}, file_id::IdManager, node::Node, require::Format};


/// ## 文件系统抽象
/// 将磁盘中的文件系统抽象为一个 System，各种格式都转换成此结构
/// 同时为文件的读写提供同步保证
/// 文件操作以文件的标志为基础进行读写，使用前先获取标志
/// 文件系统所有磁盘操作以块为基本单位
pub struct FileSystem {
    pub id_mgr : IdManager,
    pub files : BTreeMap<usize, File>,
    pub path_to_id : BTreeMap<String, usize>,
    pub cache_buffer : &'static mut dyn CacheBuffer,
    pub format : &'static mut dyn Format,
    pub total_size : usize,
    pub block_size : usize,
    pub block_start : usize,
    pub device_id : usize,
    pub root : Node,
}

impl FileSystem {
    pub fn new(
        cache_buffer:&'static mut dyn CacheBuffer,
        format : &'static mut dyn Format,
        device_id : usize,
    )->Self {
        let data = &mut [0;512];
        cache_buffer.read(device_id, data, 0);
        let info = format.parse_super_block(data);
        let root = Node::new(String::from("root"), String::from("/"),
                info.root_directory_block_idx,
            format.parse_node(info.root_directory_block_idx).unwrap());
        Self {
            id_mgr: IdManager::new(),
            files: BTreeMap::new(),
            path_to_id: BTreeMap::new(),
            cache_buffer,
            format,
            total_size: info.total_size,
            block_size: info.block_size,
            device_id,
            block_start : info.block_start_addr,
            root,
        }
    }

    pub fn open(&mut self, path : String, flag : FileFlag)->Result<&File, ()> {
        if let Some(id) = self.path_to_id.get(&path) {
            let file = self.files.get_mut(id).unwrap();
            file.open(flag).unwrap();
            Ok(file)
        }
        else {
            let leaf = self.root.search_leaf(path.clone(), self.format).unwrap();
            let file = self.generate_file(leaf, path).unwrap();
            file.open(flag).unwrap();
            Ok(file)
        }
    }

    pub fn enter(&mut self, path : String)->Result<Directory, ()> {
        let node = self.root.search_node(path, self.format).unwrap();
        self.generate_directory(node)
    }

    fn generate_directory(&mut self, node : Node)->Result<Directory, ()> {
        let mut item = Vec::new();
        for file in node.file.iter() {
            item.push(DirectoryItem {
                name: file.name.clone(),
                itype: crate::DirItemType::File,
            });
        }
        for dir in node.directory.iter() {
            item.push(DirectoryItem {
                name : dir.name.clone(),
                itype : crate::DirItemType::Directory
            })
        }
        Ok(Directory {
            name: node.name.clone(),
            block_idx: node.block_idx,
            path: node.path.clone(),
            item,
        })
    }

    fn generate_file(&mut self, leaf : Leaf, path : String)->Result<&mut File, &str> {
        if leaf.is_file() {
            if let Some(id) = self.path_to_id.get(&path) {
                let file = self.files.get_mut(id).unwrap();
                Ok(file)
            }
            else {
                let file = File {
                    id : self.id_mgr.get(),
                    device_id: self.device_id,
                    start_idx : leaf.block_idx,
                    name: leaf.name.clone(),
                    state: FileState::new(),
                    path : path.clone(),
                    size: leaf.size,
                };
                let id = file.id;
                self.path_to_id.insert(path, file.id);
                self.files.insert(file.id, file);
                Ok(self.files.get_mut(&id).unwrap())
            }
        }
        else {
            Err("Isn't a file")
        }
    }

    pub fn read(&mut self, id : usize, data : &mut [u8])->IoResult {
        if let Some(file) = self.files.get_mut(&id) {
            if file.read() {
                let leaf = self.root.search_leaf(file.path.clone(), self.format).unwrap();
                let block_chain =
                    self.format.get_block_chain(leaf.block_idx).unwrap();
                for (idx, addr) in block_chain.iter().enumerate() {
                    let st = idx * self.block_size;
                    if st >= data.len() {
                        break;
                    }
                    let ed = min((idx + 1) * self.block_size, data.len());
                    let data = &mut data[st..ed];
                    self.cache_buffer.read(self.device_id, data,
                        self.block_start + *addr * self.block_size);
                }
                Ok(())
            }
            else { Err(IoError::ReadFromWrite) }
        }
        else { Err(IoError::FileClosed) }
    }

    pub fn write(&mut self, id : usize, data : &[u8])->IoResult {
        if let Some(file) = self.files.get_mut(&id) {
            if file.write() {
                let leaf = self.root.search_leaf(file.path.clone(), self.format).unwrap();
                let block_chain =
                    self.format.get_block_chain(leaf.block_idx).unwrap();
                for (idx, addr) in block_chain.iter().enumerate() {
                    let st = idx * self.block_size;
                    if st >= data.len() {
                        break;
                    }
                    let ed = min((idx + 1) * self.block_size, data.len());
                    let data =
                        &data[st..ed];
                    self.cache_buffer.write(self.device_id, data,
                        self.block_start + *addr * self.block_size);
                }
                Ok(())
            }
            else { Err(IoError::WriteToReadOnly) }
        }
        else { Err(IoError::FileClosed) }
    }

    pub fn refresh(&mut self, dir : &Directory) {
        self.root.refresh(dir.path.clone(), self.format).unwrap();
    }

    pub fn total_size(&self)->usize {
        self.total_size
    }

    pub fn block_size(&self)->usize {
        self.block_size
    }
}


type IoResult = Result<(), IoError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IoError {
    WriteToReadOnly,
    ReadFromWrite,
    FileClosed,
}