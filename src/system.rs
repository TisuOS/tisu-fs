use core::cmp::min;

use alloc::{collections::BTreeMap, sync::Arc};
use alloc::prelude::v1::*;
use device_buffer::CacheBuffer;
use crate::{DirectoryItem, FileFlag, Leaf, SystemOp, directory::Directory, file::{File, FileState}, file_id::IdManager, node::Node, require::Format};

/// ## 文件系统抽象
/// 将磁盘中的文件系统抽象为一个 System，各种格式都转换成此结构
/// 同时为文件的读写提供同步保证
/// 文件操作以文件的标志为基础进行读写，使用前先获取标志
/// 文件系统所有磁盘操作以块为基本单位
pub struct FileSystem {
    pub id_mgr : &'static mut IdManager,
    pub files : BTreeMap<usize, File>,
    pub path_to_id : BTreeMap<String, usize>,
    pub cache_buffer : &'static mut dyn CacheBuffer,
    pub format : Arc<dyn Format>,
    pub total_size : usize,
    pub block_size : usize,
    pub block_start : usize,
    pub device_id : usize,
    pub root : Node,
}

impl FileSystem {
    pub fn new(
        cache_buffer:&'static mut dyn CacheBuffer,
        format : Arc<dyn Format>,
        id_mgr : &'static mut IdManager,
        device_id : usize,
    )->Self {
        let info = format.parse_super_block();
        let root = Node::new(String::from("root"), String::from("/"),
                info.root_directory_block_idx,
            format.parse_node(info.root_directory_block_idx).unwrap());
        Self {
            id_mgr,
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

    fn format_path(&self, path :&String, dir : bool)->String {
        let rt = path.trim().to_string();
        let mut rt = rt.trim_matches('/').to_string();
        if rt.len() > 0 && dir{
            rt.push('/');
        }
        rt
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
            device_id : self.device_id,
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
}

impl SystemOp for FileSystem {
    fn open(&mut self, path : String, flag : FileFlag)->Result<&File, ()> {
        let path = self.format_path(&path, false);
        if let Some(id) = self.path_to_id.get(&path) {
            let file = self.files.get_mut(id).unwrap();
            file.open(flag).unwrap();
            Ok(file)
        }
        else {
            let leaf = self.root.search_leaf(path.clone(), self.format.clone()).unwrap();
            let file = self.generate_file(leaf, path).unwrap();
            file.open(flag).unwrap();
            Ok(file)
        }
    }

    fn enter(&mut self, path : String)->Result<Directory, ()> {
        let path = self.format_path(&path, true);
        let node = self.root.search_node(path, self.format.clone()).unwrap();
        self.generate_directory(node)
    }

    fn get_file(&mut self, path : String)->Result<File, IoError> {
        let path = self.format_path(&path, false);
        if let Some(id) = self.path_to_id.get(&path) {
            let file = self.files.get_mut(id).unwrap();
            Ok(file.clone())
        }
        else {
            let leaf = self.root.search_leaf(path.clone(), self.format.clone()).unwrap();
            let file = self.generate_file(leaf, path).unwrap();
            Ok(file.clone())
        }
    }

    fn read(&mut self, id : usize, data : &mut [u8])->IoResult {
        if let Some(file) = self.files.get_mut(&id) {
            if file.read() {
                let leaf = self.root.search_leaf(file.path.clone(), self.format.clone()).unwrap();
                let block_chain =
                    self.format.get_block_chain(leaf.block_idx).unwrap();
                let mut len = 0;
                for (idx, addr) in block_chain.iter().enumerate() {
                    let st = idx * self.block_size;
                    if st >= data.len() {
                        break;
                    }
                    let ed = min((idx + 1) * self.block_size, data.len());
                    let data = &mut data[st..ed];
                    len += ed - st;
                    self.cache_buffer.read(self.device_id, data,
                        self.block_start + *addr * self.block_size);
                }
                Ok(len)
            }
            else { Err(IoError::ReadFromWrite) }
        }
        else { Err(IoError::FileClosed) }
    }

    fn write(&mut self, id : usize, data : &[u8])->IoResult {
        if let Some(file) = self.files.get_mut(&id) {
            if file.write() {
                let leaf = self.root.search_leaf(file.path.clone(), self.format.clone()).unwrap();
                let block_chain =
                    self.format.get_block_chain(leaf.block_idx).unwrap();
                let mut len = 0;
                for (idx, addr) in block_chain.iter().enumerate() {
                    let st = idx * self.block_size;
                    if st >= data.len() {
                        break;
                    }
                    let ed = min((idx + 1) * self.block_size, data.len());
                    let data = &data[st..ed];
                    len += ed - st;
                    self.cache_buffer.write(self.device_id, data,
                        self.block_start + *addr * self.block_size);
                }
                Ok(len)
            }
            else { Err(IoError::WriteToReadOnly) }
        }
        else { Err(IoError::FileClosed) }
    }

    fn refresh(&mut self, dir : &Directory) {
        self.root.refresh(dir.path.clone(), self.format.clone()).unwrap();
    }

    fn total_size(&self)->usize {
        self.total_size
    }

    fn block_size(&self)->usize {
        self.block_size
    }

    fn contain(&self, id : usize)->bool {
        self.files.contains_key(&id)
    }

    fn check(&self) ->usize {
        self.format.get_device()
    }
}


pub type IoResult = Result<usize, IoError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IoError {
    WriteToReadOnly,
    ReadFromWrite,
    FileClosed,
}