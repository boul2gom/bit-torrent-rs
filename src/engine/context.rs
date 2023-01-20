use std::path::PathBuf;
use crate::shared::{SizedBytes, SyncResult};
use crate::types::bencode::MetaInfoFile;
use crate::utils::data::manipulator;

pub struct EngineContext {
    pub name: String,
    pub announce: String,

    pub info_hash: String,

    pub piece_length: u32,
    pub length: u32,
    pub pieces: Vec<SizedBytes>,

    pub destination: tokio::fs::File,
}

impl EngineContext {
    pub async fn new(meta_info: MetaInfoFile, length: u32, destination: PathBuf) -> SyncResult<Self> {
        let name = meta_info.info.name.clone();
        let announce = meta_info.announce.clone();
        let piece_length = meta_info.info.piece_length;

        let destination = tokio::fs::File::open(destination).await?;

        Ok(Self {
            name,
            announce,
            info_hash: manipulator::hash_meta_info(&meta_info)?,
            piece_length,
            length,
            pieces: manipulator::split_piece_bytes(&meta_info)?,
            destination,
        })
    }
}