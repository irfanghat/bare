use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};

use crate::format::{AdjacencyRange, HEADER_SIZE, Header, INDEX_ENTRY_SIZE, MAGIC, VERSION};

pub struct BareWriter {
    file: File,
    ranges: Vec<AdjacencyRange>,
}

impl BareWriter {
    pub fn create(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let mut file = File::create(path)?;

        file.write_all(&[0u8; 32])?;

        Ok(Self {
            file,
            ranges: Vec::new(),
        })
    }

    pub fn add_vertex(&mut self, vertex_id: u64, neighbors: &[u64]) -> io::Result<()> {
        if vertex_id as usize != self.ranges.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Vertices must be added sequentially",
            ));
        }

        let offset = self.file.stream_position()?;

        self.file
            .write_all(&(neighbors.len() as u64).to_le_bytes())?;

        for neighbor in neighbors {
            self.file.write_all(&neighbor.to_le_bytes())?;
        }

        let end = self.file.stream_position()?;

        self.ranges.push(AdjacencyRange {
            offset,
            length: end - offset,
        });

        Ok(())
    }

    pub fn finish(mut self) -> io::Result<()> {
        let index_offset = self.file.stream_position()?;

        for range in &self.ranges {
            self.file.write_all(&range.offset.to_le_bytes())?;
            self.file.write_all(&range.length.to_le_bytes())?;
        }

        let data_offset = HEADER_SIZE;

        let header = Header {
            version: VERSION,
            vertex_count: self.ranges.len() as u64,
            index_offset,
            data_offset,
        };

        self.file.seek(SeekFrom::Start(0))?;

        self.file.write_all(MAGIC)?;
        self.file.write_all(&header.version.to_le_bytes())?;
        self.file.write_all(&header.vertex_count.to_le_bytes())?;
        self.file.write_all(&header.index_offset.to_le_bytes())?;
        self.file.write_all(&header.data_offset.to_le_bytes())?;

        self.file.flush()?;

        Ok(())
    }
}
