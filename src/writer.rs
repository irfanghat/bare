use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};

use crate::format::{AdjacencyRange, Header, INDEX_ENTRY_SIZE, MAGIC, VERSION};

pub struct BareWriter {
    file: File,
    ranges: Vec<AdjacencyRange>,
}
