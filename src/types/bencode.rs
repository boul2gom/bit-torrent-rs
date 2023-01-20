use std::path::PathBuf;
use serde_bytes::ByteBuf;
use serde_derive::{Serialize, Deserialize};
use crate::shared::SyncResult;

#[derive(Clone, Serialize, Deserialize)]
pub struct MetaInfoFile {
    pub info: MetaInfoDictionary,
    pub announce: String,

    #[serde(rename = "announce-list", skip_serializing_if = "Option::is_none")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "created by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetaInfoDictionary {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: u32,
    pub pieces: ByteBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<u32>,

    //Single file mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,

    //Multi file mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<MetaInfoFileEntry>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetaInfoFileEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TrackerResponse {
    #[serde(rename = "failure reason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "warning message", skip_serializing_if = "Option::is_none")]
    pub warning_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
    #[serde(rename = "min interval", skip_serializing_if = "Option::is_none")]
    pub min_interval: Option<u32>,
    #[serde(rename = "tracker id", skip_serializing_if = "Option::is_none")]
    pub tracker_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<ByteBuf>,
}

impl MetaInfoFile {
    pub async fn from_file(meta_info: PathBuf) -> SyncResult<Self> {
        let raw_file = tokio::fs::read(meta_info).await?;
        let meta_info = serde_bencode::from_bytes(&raw_file)?;

        Ok(meta_info)
    }

    pub fn is_single_file_mode(&self) -> bool {
        self.info.length.is_some() && self.info.files.is_none()
    }

    pub fn is_multi_file_mode(&self) -> bool {
        self.info.length.is_none() && self.info.files.is_some()
    }
}