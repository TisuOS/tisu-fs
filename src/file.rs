/// ## 文件
/// 文件结构体作为文件的唯一合理存在标志
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
            state:self.state,
            size:self.size,
        }
    }
}

impl File {
    pub fn open(&mut self, flag : FileFlag)->Result<(), FileError> {
        if self.state.close() {
            self.state.set(flag);
            Ok(())
        }
        else {
            match flag {
                FileFlag::Read =>
                    if self.state.read() {Ok(())} else {Err(FileError::ReadFromWrite)},
                FileFlag::Write =>
                    if self.state.write() {Ok(())} else {Err(FileError::WriteToReadOnly)},
                _ => Err(FileError::FlagErr(flag)),
            }
        }
    }

    pub fn read(&self)->bool {
        self.state.read()
    }

    pub fn write(&self)->bool {
        self.state.write()
    }

    pub fn read_only(&self)->bool {
        self.state.read_only()
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
}

#[derive(Clone, Copy, Debug)]
pub struct FileState {
    pub flag : FileFlag,
}

impl FileState {
    pub fn new()->Self {
        Self {
            flag: FileFlag::Close
        }
    }

    pub fn close(&self)->bool {
        self.flag == FileFlag::Close
    }

    pub fn set(&mut self, flag : FileFlag) {
        self.flag = flag;
    }

    pub fn read(&self)->bool {
        self.flag == FileFlag::Read || self.flag == FileFlag::ReadWrite
    }

    pub fn read_only(&self)->bool {
        self.flag == FileFlag::Read
    }

    pub fn write(&self)->bool {
        self.flag == FileFlag::Write || self.flag == FileFlag::ReadWrite
    }
}


use alloc::prelude::v1::*;
