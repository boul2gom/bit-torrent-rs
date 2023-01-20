use crate::shared::{PEER_ID, SizedBytes, SyncResult};

#[derive(Clone, PartialEq)]
pub enum MessageCode {
    MessageChoke = 0,
    MessageUnchoke = 1,
    MessageInterested = 2,
    MessageNotInterested = 3,
    MessageHave = 4,
    MessageBitfield = 5,
    MessageRequest = 6,
    MessagePiece = 7,
    MessageCancel = 8,
    //Keep-alive message
    MessageKeepAlive = 254,
    //Rust needs a way to specify the last value in an enum
    MessageUnknown = 255,
}

pub struct Message {
    pub id: MessageCode,
    pub payload: Vec<u8>,
}

pub struct Handshake {
    pub pstr: String,
    pub info_hash: String,
    pub peer_id: SizedBytes,
}

impl Message {
    pub fn new(id: MessageCode, payload: Vec<u8>) -> Message {
        Message {
            id,
            payload,
        }
    }
}

impl Handshake {
    pub fn new(info_hash: String) -> SyncResult<Handshake> {
        let peer_id = PEER_ID.get().ok_or("Failed to get peer id, is it set ?")?.clone();

        Ok(Handshake {
            pstr: "BitTorrent protocol".to_string(),
            info_hash,
            peer_id,
        })
    }
}

impl From<u8> for MessageCode {
    fn from(id: u8) -> Self {
        match id {
            0 => MessageCode::MessageChoke,
            1 => MessageCode::MessageUnchoke,
            2 => MessageCode::MessageInterested,
            3 => MessageCode::MessageNotInterested,
            4 => MessageCode::MessageHave,
            5 => MessageCode::MessageBitfield,
            6 => MessageCode::MessageRequest,
            7 => MessageCode::MessagePiece,
            8 => MessageCode::MessageCancel,
            254 => MessageCode::MessageKeepAlive,
            _ => MessageCode::MessageUnknown,
        }
    }
}

impl Into<u8> for MessageCode {
    fn into(self) -> u8 {
        self as u8
    }
}