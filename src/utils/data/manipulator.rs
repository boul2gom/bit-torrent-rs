use sha1::{Digest, Sha1};
use crate::shared::{SizedBytes, SyncResult};
use crate::types::bencode::MetaInfoFile;

pub fn hash_meta_info(to_hash: &MetaInfoFile) -> SyncResult<String> {
    let encoded = serde_bencode::to_bytes(&to_hash.info)?;
    let digest = Sha1::digest(&encoded);

    let mut info_hash = [0; 20];
    info_hash.copy_from_slice(&digest);

    let hex_encoded = hex::encode(&info_hash);

    Ok(hex_encoded)
}

pub fn split_piece_bytes(to_split: &MetaInfoFile) -> SyncResult<Vec<SizedBytes>> {
    let mut pieces = Vec::new();

    let raw_pieces = to_split.info.pieces.as_ref();
    println!("[split_piece_bytes] Raw pieces size: {}", raw_pieces.len());
    let piece_length = to_split.info.piece_length;
    println!("[split_piece_bytes] Piece length: {}", piece_length);

    let mut index = 0;
    while index < raw_pieces.len() {
        let mut piece = [0; 20];

        piece.copy_from_slice(&raw_pieces[index..index + 20]);
        pieces.push(piece);

        index += piece_length as usize;
    }

    Ok(pieces)
}