use std::net::IpAddr;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
}

impl Peer {
    pub fn new(ip: IpAddr, port: u16) -> Peer {
        Peer {
            ip,
            port,
        }
    }
}