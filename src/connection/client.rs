use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::shared::SyncResult;
use crate::types::bitfield::BitField;
use crate::types::message::{Handshake, Message, MessageCode};
use crate::types::peer::Peer;

pub struct Client {
    pub connection: TcpStream,
    pub choked: bool,

    pub bitfield: BitField,
    pub peer: Peer,
    pub info_hash: String,
}

impl Client {
    pub async fn connect(peer: Peer, info_hash: String) -> SyncResult<Client> {
        let address = SocketAddr::new(peer.ip, peer.port);
        println!("[Client - connect] Socket address built");
        let mut connection = TcpStream::connect(address).await?;
        println!("[Client - connect] Connection established");

        let _handshake = Client::complete_handshake(&mut connection, info_hash.clone()).await?;
        println!("[Client - connect] Handshake completed");
        let bitfield = Client::receive_bitfield(&mut connection).await?;
        println!("[Client - connect] Bitfield received");

        let client = Client {
            connection,
            choked: false,

            bitfield,
            peer,
            info_hash,
        };

        Ok(client)
    }

    pub async fn complete_handshake(connection: &mut TcpStream, info_hash: String) -> SyncResult<Handshake> {
        let handshake = Handshake::new(info_hash.clone())?;
        println!("[Client - complete_handshake] Handshake built");
        let bytes = handshake.to_bytes()?;
        println!("[Client - complete_handshake] Handshake bytes built");

        connection.write_all(&bytes).await?;
        println!("[Client - complete_handshake] Handshake bytes written");

        let handshake = Client::read_handshake(connection).await?;
        println!("[Client - complete_handshake] Handshake read");

        if handshake.pstr != "BitTorrent protocol" {
            return Err("Invalid pstr in handshake".into());
        }

        if handshake.info_hash != info_hash {
            return Err("Invalid info_hash in handshake".into());
        }

        Ok(handshake)
    }

    pub async fn read_handshake(connection: &mut TcpStream) -> SyncResult<Handshake> {
        let mut buffer = [0; 1];
        println!("[Client - read_handshake] Buffer initialized");
        connection.read_exact(&mut buffer).await?;
        println!("[Client - read_handshake] pstrlen read");

        let pstr_len = buffer[0] as usize;
        println!("[Client - read_handshake] pstrlen is: {}", pstr_len);

        let mut buffer = vec![0; pstr_len + 20 + 20 + 8];
        connection.read_exact(&mut buffer).await?;
        println!("[Client - read_handshake] Read handshake");

        let handshake = Handshake::from_bytes(pstr_len, &buffer)?;
        println!("[Client - read_handshake] Handshake built from bytes");

        Ok(handshake)
    }

    pub async fn receive_bitfield(connection: &mut TcpStream) -> SyncResult<BitField> {
        let message = Client::static_read_message(connection).await?;
        println!("[Client - receive_bitfield] Message read");

        println!("[Client - receive_bitfield] Received message id: {}", <MessageCode as Into<u8>>::into(message.id.clone()));
        if message.id != MessageCode::MessageBitfield {
            return Err("Received non-bitfield message when expecting bitfield".into());
        }

        let bitfield = BitField::new(message.payload);

        Ok(bitfield)
    }

    pub async fn static_read_message(connection: &mut TcpStream) -> SyncResult<Message> {
        let mut buffer = [0; 4];
        connection.read_exact(&mut buffer).await?;

        let length = u32::from_be_bytes(buffer);

        //Keep-alive message
        if length == 0 {
            return Ok(Message::new(MessageCode::MessageKeepAlive, vec![]));
        }

        let mut buffer = vec![0; length as usize];
        connection.read_exact(&mut buffer).await?;

        let message = Message::from_bytes(&buffer)?;

        Ok(message)
    }

    pub async fn read_message(&mut self) -> SyncResult<Message> {
        Client::static_read_message(&mut self.connection).await
    }

    pub async fn send_message(&mut self, message: Message) -> SyncResult<()> {
        let bytes = message.to_bytes()?;

        self.connection.write_all(&bytes).await?;

        Ok(())
    }

    pub async fn send_interested(&mut self) -> SyncResult<()> {
        let message = Message::new(MessageCode::MessageInterested, Vec::new());

        self.send_message(message).await
    }

    pub async fn send_not_interested(&mut self) -> SyncResult<()> {
        let message = Message::new(MessageCode::MessageNotInterested, Vec::new());

        self.send_message(message).await
    }

    pub async fn send_unchoke(&mut self) -> SyncResult<()> {
        let message = Message::new(MessageCode::MessageUnchoke, Vec::new());

        self.send_message(message).await
    }

    pub async fn send_request(&mut self, index: u32, begin: u32, length: u32) -> SyncResult<()> {
        let message = Message::format_request(index, begin, length);

        self.send_message(message).await
    }

    pub async fn send_have(&mut self, index: u32) -> SyncResult<()> {
        let message = Message::format_have(index);

        self.send_message(message).await
    }
}