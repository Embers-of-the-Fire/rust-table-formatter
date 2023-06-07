use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TableError {
    #[error("Invalid table size - row {row}: find {actual} cells, expect {expected}")]
    InvalidTableSize {
        row: usize,
        expected: usize,
        actual: usize,
    },
    #[error("Cell Merge out of table - ({x} {y}): merging {merge} cells, maximum {max} cells")]
    MergeOutOfTable {
        x: usize,
        y: usize,
        merge: usize,
        max: usize,
    },
    #[error("Merge overwriting cells: ({x} {y})")]
    MergeOverwrite { x: usize, y: usize },
    #[error("Write to buffer failed: {0}")]
    WriteError(io::Error),
    #[error("Unexpected Error: {0}; please report to the author")]
    Unexpected(String),
}

impl From<io::Error> for TableError {
    fn from(value: io::Error) -> Self {
        Self::WriteError(value)
    }
}
