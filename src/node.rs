use alloc::{collections::BTreeMap, prelude::v1::*, sync::Arc};

use crate::{leaf::Leaf, require::Format};

pub struct Node {
    pub name : String,
    pub path : String,
    pub block_idx : usize,
    pub directory : Vec<Leaf>,
    pub file : Vec<Leaf>,
    pub node : Option<BTreeMap<String, Node>>,
}

impl Node {
    pub fn init(&mut self, format: &mut dyn Format) {
        let leaf = format.parse_node(self.block_idx).unwrap();
        for l in leaf {
            if l.is_directory() {
                self.directory.push(l);
            }
            else {
                self.file.push(l);
            }
        }
    }

    pub fn search_leaf(&mut self, path : String, format : Arc<dyn Format>)->Result<Leaf, NodeError> {
        if !path.contains("/") {
            if let Some(dir) = self.directory.iter().find(|d|{d.name == path}) {
                return Ok(dir.clone());
            }
            else if let Some(file) = self.file.iter().find(|f|{f.name == path}) {
                return Ok(file.clone());
            }
            else {
                return Err(NodeError::NoDirectory(path));
            }
        }
        let (name, p) = path.split_once("/").unwrap();
        for dir in self.directory.iter_mut() {
            if dir.name == *name {
                self.expend(format.clone());
                if let Some(node) = &mut self.node {
                    return node.get_mut(name).unwrap().search_leaf(p.to_string(), format);
                }
                return Err(NodeError::ExpendErr);
            }
        }
        Err(NodeError::NoDirectory("s".to_string() + &path[..] + " " + name + " " + p))
    }

    pub fn search_node(&mut self, path : String, format : Arc<dyn Format>)->Result<Node, NodeError> {
        if path.len() == 0 {
            self.expend(format);
            return Ok(Node {
                name : self.name.clone(),
                path : self.path.clone(),
                block_idx : self.block_idx,
                directory : self.directory.clone(),
                file : self.file.clone(),
                node : None,
            });
        }
        let (name, p) = path.split_once("/").unwrap();
        for dir in self.directory.iter_mut() {
            if dir.name == *name {
                self.expend(format.clone());
                if let Some(node) = &mut self.node {
                    return node.get_mut(name).unwrap().search_node(p.to_string(), format);
                }
                return Err(NodeError::ExpendErr);
            }
        }
        Err(NodeError::NoDirectory("s".to_string() + &path[..] + " " + name + " " + p))
    }

    fn expend(&mut self, format : Arc<dyn Format>) {
        if self.node.is_none() {
            let mut nodes = BTreeMap::new();
            for dir in self.directory.iter() {
                let path = self.path.clone() + &dir.name[..];
                nodes.insert(dir.name.clone(),
                Node::new(dir.name.clone(), path, dir.block_idx,
                    format.parse_node(dir.block_idx).unwrap()));
            }
            self.node = Some(nodes)
        }
    }

    pub fn refresh(&mut self, path:String, format : Arc<dyn Format>)->Result<(), NodeError> {
        if path.len() == 0 {
            self.reset(format);
            return Ok(())
        }
        let (name, path) = path.split_once("/").unwrap();
        for dir in self.directory.iter_mut() {
            if dir.name == *name {
                self.expend(format.clone());
                if let Some(node) = &mut self.node {
                    return node.get_mut(name).unwrap().refresh(path.to_string(), format);
                }
                return Err(NodeError::ExpendErr);
            }
        }
        Err(NodeError::NoDirectory(path.to_string()))
    }

    pub fn reset(&mut self, format : Arc<dyn Format>) {
        let leaves = format.parse_node(self.block_idx).unwrap();
        for leaf in leaves {
            match leaf.ltype {
                crate::leaf::LeafType::File => self.file.push(leaf),
                crate::leaf::LeafType::Directory => {
                    self.directory.push(leaf);
                }
            }
        }
        self.node = None;
    }

    pub fn new(name : String, path: String, block_idx: usize, leaf : Vec<Leaf>)->Self {
        let mut file = Vec::new();
        let mut directory = Vec::new();
        for l in leaf {
            if l.is_directory() { directory.push(l) }
            else { file.push(l) }
        }
        Self {
            name,
            path,
            block_idx,
            directory,
            file,
            node: None,
        }
    }
}

pub enum ItemType {
    Leaf(Leaf),
    Node(Node),
}

#[derive(Debug)]
pub enum NodeError {
    NoFile(String),
    NoDirectory(String),
    ExpendErr,
}