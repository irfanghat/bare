pub const MAGIC: &[u8; 4] = b"BARE";
pub const VERSION: u32 = 1;

pub const HEADER_SIZE: u64 = 32;
pub const INDEX_ENTRY_SIZE: u64 = 16;

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub version: u32,
    pub vertex_count: u64,
    pub index_offset: u64,
    pub data_offset: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjacencyRange {
    pub offset: u64,
    pub length: u64,
}
