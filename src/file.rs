/// ## 文件
/// 文件结构体作为文件的信息体现
#[derive(Debug)]
pub struct File{
    pub id : usize,
    pub device_id : usize,
    pub start_idx : usize,
    pub name : String,
    pub path : String,
    pub state : FileState,
    pub size : usize,
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self {
            id : self.id,
            device_id:self.device_id,
            start_idx : self.start_idx,
            name:self.name.clone(),
            path : self.path.clone(),
            state:self.state.clone(),
            size:self.size,
        }
    }
}

impl File {
    pub fn open(&mut self, flag : FileFlag)->Result<(), FileError> {
        if self.state.is_close() {
            self.state.set(flag);
            Ok(())
        }
        else {
            match flag {
                FileFlag::Read =>
                    if !self.state.writable() {Ok(())} else {Err(FileError::ReadFromWrite)},
                FileFlag::Write =>
                    if self.state.is_close() {Ok(())} else {Err(FileError::WriteToReadOnly)},
                _ => Err(FileError::FlagErr(flag)),
            }
        }
    }

    pub fn readable(&self)->bool {
        self.state.readable()
    }

    pub fn writable(&self)->bool {
        self.state.writable()
    }

    pub fn read_only(&self)->bool {
        self.state.read_only()
    }

    pub fn close(&mut self) {
        self.state.flag = FileFlag::Close;
    }

    pub fn is_own(&self, task_id : usize)->bool {
        self.state.is_own(task_id)
    }

    pub fn own(&mut self, task_id : usize) {
        self.state.own(task_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileError {
    WriteToReadOnly,
    ReadFromWrite,
    FlagErr(FileFlag),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileFlag{
    Close = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
}

impl FileFlag{
    pub fn val(self)->usize{
        self as usize
    }

    pub fn from(n : usize)->Option<Self> {
        match n {
            1 => Some(Self::Read),
            2 => Some(Self::Write),
            3 => Some(Self::ReadWrite),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct FileState {
    pub flag : FileFlag,
    pub owner : Vec<usize>,
}

impl FileState {
    pub fn new()->Self {
        Self {
            flag: FileFlag::Close,
            owner:Vec::new(),
        }
    }

    pub fn is_close(&self)->bool {
        self.flag == FileFlag::Close
    }

    pub fn set(&mut self, flag : FileFlag) {
        self.flag = flag;
    }

    pub fn readable(&self)->bool {
        self.flag == FileFlag::Read || self.flag == FileFlag::ReadWrite
    }

    pub fn read_only(&self)->bool {
        self.flag == FileFlag::Read
    }

    pub fn writable(&self)->bool {
        self.flag == FileFlag::Write || self.flag == FileFlag::ReadWrite
    }

    pub fn is_own(&self, task_id : usize)->bool {
        self.owner.contains(&task_id)
    }

    pub fn own(&mut self, task_id : usize) {
        self.owner.push(task_id)
    }
}

impl Clone for FileState {
    fn clone(&self) -> Self {
        Self {
            flag:self.flag,
            owner:self.owner.clone(),
        }
    }
}


use alloc::prelude::v1::*;
