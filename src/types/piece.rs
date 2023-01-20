use serde_derive::{Serialize, Deserialize};
use crate::shared::SizedBytes;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PieceWork {
    pub index: u32,
    pub hash: SizedBytes,
    pub length: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PieceResult {
    pub index: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PieceProgress {
    pub index: u32,
    pub data: Vec<u8>,
    pub downloaded: u32,
    pub requested: u32,
    pub backlog: u32,
}

impl PieceWork {
    pub fn new(index: u32, hash: SizedBytes, length: u32) -> PieceWork {
        PieceWork {
            index,
            hash,
            length,
        }
    }
}

impl PieceResult {
    pub fn new(index: u32, data: Vec<u8>) -> PieceResult {
        PieceResult {
            index,
            data,
        }
    }
}

impl PieceProgress {
    pub fn new(index: u32) -> PieceProgress {
        PieceProgress {
            index,
            data: Vec::new(),
            downloaded: 0,
            requested: 0,
            backlog: 0,
        }
    }
}