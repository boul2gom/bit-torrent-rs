use std::net::{IpAddr, Ipv4Addr};
use crate::shared::{PEER_SIZE, SyncResult};
use crate::types::message::{Handshake, Message, MessageCode};
use crate::types::peer::Peer;

impl Message {
    pub fn to_bytes(&self) -> SyncResult<Vec<u8>> {
        //Keep-alive message
        if self.id == MessageCode::MessageKeepAlive || self.payload.is_empty() {
            return Ok(vec![0; 4]);
        }

        let mut bytes = Vec::with_capacity(4 + 1 + self.payload.len());
        //Append payload length as 4 bytes, including the id byte
        bytes.extend_from_slice(&(self.payload.len() as u32 + 1).to_be_bytes());
        //Append id as 1 byte
        let byte_id = self.id.clone().into();
        bytes.push(byte_id);
        //Append payload
        bytes.extend_from_slice(&self.payload);

        Ok(bytes)
    }

    pub fn from_bytes(buf: &[u8]) -> SyncResult<Message> {
        let id = buf[0];
        let payload = buf[1..].to_vec();

        Ok(Message::new(id.into(), payload))
    }
}

impl Handshake {
    pub fn to_bytes(&self) -> SyncResult<Vec<u8>> {
        let mut buf = Vec::new();

        buf.push(self.pstr.len() as u8);
        buf.extend(self.pstr.as_bytes());
        buf.extend(vec![0; 8]);
        buf.extend(hex::decode(&self.info_hash)?);
        buf.extend(&self.peer_id);

        Ok(buf)
    }

    pub fn from_bytes(pstr_len: usize, buf: &[u8]) -> SyncResult<Handshake> {
        let buffer_end = pstr_len;
        let pstr = std::str::from_utf8(&buf[0..buffer_end])?;

        let buffer_start = pstr_len + 8;
        let buffer_end = pstr_len + 20 + 8;
        let info_hash = hex::encode(&buf[buffer_start..buffer_end]);

        let buffer_start = pstr_len + 20 + 8;
        let buffer_end = pstr_len + 20 + 20 + 8;
        let peer_id = buf[buffer_start..buffer_end].try_into()?;

        Ok(Handshake {
            pstr: pstr.to_string(),
            info_hash,
            peer_id,
        })
    }
}

impl Peer {
    pub fn from_bytes(bytes: &[u8]) -> SyncResult<Vec<Peer>> {
        let peer_length = bytes.len();
        if peer_length % PEER_SIZE as usize != 0 {
            return Err("Peer length is not a multiple of 6".into());
        }

        let mut peers = Vec::new();

        let mut index = 0;
        while index < peer_length {
            let ip = Ipv4Addr::new(bytes[index], bytes[index + 1], bytes[index + 2], bytes[index + 3]);
            let port = u16::from_be_bytes([bytes[index + 4], bytes[index + 5]]);

            let peer = Peer {
                ip: IpAddr::V4(ip),
                port
            };
            peers.push(peer);

            index += PEER_SIZE as usize;
        }

        Ok(peers)
    }
}