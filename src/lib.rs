//! # 太素文件系统抽象
//! 文件系统被抽象为两个部分：树状管理结构 System；信息结构 Directory、File
//! System 负责具体的操作，同时管理改文件系统内的所有文件
//! 其中的 Node 作为文件树节点保管目录信息，Leaf 象征着目录中的每一个项
//! Directory、File 没有任何操作能力，作为信息载体与外界交互
//! 信息结构用于外界交流
//!
//! 2021年4月23日 zg

#![no_std]
#![feature(
    alloc_prelude,
)]
extern crate alloc;
mod require;
mod system;
mod file;
mod directory;
mod node;
mod leaf;
mod file_id;
mod disk_info;

pub use directory::*;
pub use file::{File, FileFlag};
pub use system::FileSystem;
pub use require::*;
pub use file_id::IdManager;
pub use leaf::*;
pub use node::*;
pub use disk_info::*;