use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::format::{AdjacencyRange, Header, INDEX_ENTRY_SIZE, MAGIC};

pub struct BareReader {
    file: File,
    header: Header,
}

impl BareReader {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let mut file = File::open(path)?;

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;

        if &magic != MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid BARE magic",
            ));
        }

        let version = read_u32(&mut file)?;
        let vertex_count = read_u64(&mut file)?;
        let index_offset = read_u64(&mut file)?;
        let data_offset = read_u64(&mut file)?;

        Ok(Self {
            file,
            header: Header {
                version,
                vertex_count,
                index_offset,
                data_offset,
            },
        })
    }

    pub fn adjacency_range(&mut self, vertex_id: u64) -> io::Result<AdjacencyRange> {
        if vertex_id >= self.header.vertex_count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "vertex does not exist",
            ));
        }

        let position = self.header.index_offset + vertex_id * INDEX_ENTRY_SIZE;

        self.file.seek(SeekFrom::Start(position))?;

        let offset = read_u64(&mut self.file)?;
        let length = read_u64(&mut self.file)?;

        Ok(AdjacencyRange { offset, length })
    }

    pub fn neighbors(&mut self, vertex_id: u64) -> io::Result<Vec<u64>> {
        let range = self.adjacency_range(vertex_id)?;

        self.file.seek(SeekFrom::Start(range.offset))?;

        let count = read_u64(&mut self.file)?;

        let mut neighbors = Vec::with_capacity(count as usize);

        for _ in 0..count {
            neighbors.push(read_u64(&mut self.file)?);
        }

        Ok(neighbors)
    }
}

fn read_u32(file: &mut File) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_u64(file: &mut File) -> io::Result<u64> {
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer))
}
