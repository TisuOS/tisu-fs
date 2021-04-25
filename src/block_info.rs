pub struct DiskInfo {
    pub stype : SystemType,
    pub total_size: usize,
    pub block_size: usize,
    pub root_directory_block_idx : usize,
    /// 第一个块开始的地址
    pub block_start_addr: usize,
}

pub enum SystemType {
    FAT32,
    Tianmu,
    Unknown,
}