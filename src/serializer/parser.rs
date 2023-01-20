use crate::connection::client::Client;
use crate::shared::SyncResult;
use crate::types::message::{Message, MessageCode};
use crate::types::piece::PieceProgress;

impl Message {
    pub fn parse_piece(&self, index: u32) -> SyncResult<Vec<u8>> {
        let payload = self.payload.as_slice();

        if self.id != MessageCode::MessagePiece {
            return Err("Message is not a piece".into());
        }

        if self.payload.len() < 8 {
            return Err("Message payload is too short".into());
        }

        let parsed_index = u32::from_ne_bytes(payload[0..4].try_into()?);
        if parsed_index != index {
            return Err("Piece index does not match".into());
        }

        let begin = u32::from_ne_bytes(payload[4..8].try_into()?);
        if begin >= self.payload.len() as u32 {
            return Err("Begin is out of bounds".into());
        }

        Ok(self.payload[8..].to_vec())
    }

    pub fn parse_have(&self) -> SyncResult<u32> {
        let payload = self.payload.as_slice();

        if self.id != MessageCode::MessageHave {
            return Err("Message is not a have".into());
        }

        if self.payload.len() != 4 {
            return Err("Message payload is not 4 bytes".into());
        }

        let index = u32::from_ne_bytes(payload[0..4].try_into()?);
        Ok(index)
    }
}

impl PieceProgress {
    pub async fn parse_message(&mut self, client: &mut Client) -> SyncResult<()> {
        let message = client.read_message().await?;
        if message.id == MessageCode::MessageKeepAlive {
            return Ok(());
        }

        match message.id {
            MessageCode::MessageUnchoke => {
                println!("[PieceProgress - parse_message] Received unchoke");
                client.choked = false
            },
            MessageCode::MessageChoke => {
                println!("[PieceProgress - parse_message] Received choke");
                client.choked = true
            },
            MessageCode::MessageHave => {
                let index = message.parse_have()?;

                println!("[PieceProgress - parse_message] Received have for piece {}", index);
                client.bitfield.set_piece(index);
            },
            MessageCode::MessagePiece => {
                println!("[PieceProgress - parse_message] Received piece message");
                let data = message.parse_piece(self.index)?;
                let length = data.len() as u32;
                self.data.extend(data);

                self.downloaded += length;
                self.backlog -= 1;

                println!("[PieceProgress - parse_message] [Piece {}] Downloaded: {}", self.index, self.downloaded);
                println!("[PieceProgress - parse_message] [Piece {}] Backlog: {}", self.index, self.backlog);
            },
            _ => {
                println!("[PieceProgress - parse_message] Received unexpected message: {}", <MessageCode as Into<u8>>::into(message.id));
            }
        }

        Ok(())
    }
}