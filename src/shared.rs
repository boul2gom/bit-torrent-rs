use once_cell::sync::OnceCell;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};

pub type SizedBytes = [u8; 20];
pub type SyncResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub static PEER_ID: OnceCell<SizedBytes> = OnceCell::new();
pub const URL_ENCODE_RESERVED: AsciiSet = NON_ALPHANUMERIC.remove(b'-').remove(b'_').remove(b'~').remove(b'.');

pub const MAX_BLOCK_SIZE: u32 = 16384;
pub const MAX_BACKLOG: u32 = 5;
pub const PEER_SIZE: u32 = 6;