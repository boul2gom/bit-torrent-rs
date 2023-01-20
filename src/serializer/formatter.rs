use crate::types::message::{Message, MessageCode};

impl Message {
    pub fn format_request(index: u32, begin: u32, length: u32) -> Message {
        let mut request = Vec::new();

        request.extend_from_slice(&index.to_be_bytes());
        request.extend_from_slice(&begin.to_be_bytes());
        request.extend_from_slice(&length.to_be_bytes());

        Message::new(MessageCode::MessageRequest, request)
    }

    pub fn format_have(index: u32) -> Message {
        let mut have = Vec::new();

        have.extend_from_slice(&index.to_be_bytes());

        Message::new(MessageCode::MessageHave, have)
    }
}