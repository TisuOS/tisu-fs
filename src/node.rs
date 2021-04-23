use alloc::{collections::BTreeMap, prelude::v1::*};

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

    pub fn search_leaf(&mut self, path : String, format : &mut dyn Format)->Result<Leaf, NodeError> {
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
        let (name, path) = path.split_once("/").unwrap();
        for dir in self.directory.iter_mut() {
            if dir.name == *name {
                self.expend(format);
                if let Some(node) = &mut self.node {
                    return node.get_mut(name).unwrap().search_leaf(path.to_string(), format);
                }
                return Err(NodeError::ExpendErr);
            }
        }
        Err(NodeError::NoDirectory("s".to_string()))
    }

    pub fn search_node(&mut self, path : String, format : &mut dyn Format)->Result<Node, NodeError> {
        if !path.contains("/") {
            self.expend(format);
            if let Some(node) = &self.node {
                let node = node.get(&path).unwrap();
                return Ok(Node {
                    name : node.name.clone(),
                    path : node.path.clone(),
                    block_idx : node.block_idx,
                    directory : node.directory.clone(),
                    file : node.file.clone(),
                    node : None,
                });
            }
            else {
                return Err(NodeError::NoDirectory(path));
            }
        }
        let (name, path) = path.split_once("/").unwrap();
        for dir in self.directory.iter_mut() {
            if dir.name == *name {
                self.expend(format);
                if let Some(node) = &mut self.node {
                    return node.get_mut(name).unwrap().search_node(path.to_string(), format);
                }
                return Err(NodeError::ExpendErr);
            }
        }
        Err(NodeError::NoDirectory("s".to_string()))
    }

    fn expend(&mut self, format : &mut dyn Format) {
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

    pub fn refresh(&mut self, path:String, format : &mut dyn Format)->Result<(), NodeError> {
        if path.len() == 0 {
            self.reset(format);
            return Ok(())
        }
        let (name, path) = path.split_once("/").unwrap();
        for dir in self.directory.iter_mut() {
            if dir.name == *name {
                self.expend(format);
                if let Some(node) = &mut self.node {
                    return node.get_mut(name).unwrap().refresh(path.to_string(), format);
                }
                return Err(NodeError::ExpendErr);
            }
        }
        Err(NodeError::NoDirectory(path.to_string()))
    }

    pub fn reset(&mut self, format : &mut dyn Format) {
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