use percent_encoding::percent_encode;
use reqwest::Client;
use crate::engine::context::EngineContext;
use crate::shared::{PEER_ID, SyncResult, URL_ENCODE_RESERVED};
use crate::types::bencode::TrackerResponse;
use crate::types::peer::Peer;

pub fn build_tracker_url(context: &EngineContext) -> SyncResult<(String, Vec<(&str, String)>)> {
    println!("[build_tracker_url] Encoded hex info_hash: {}", percent_encode(context.info_hash.as_ref(), &URL_ENCODE_RESERVED).to_string());
    println!("[build_tracker_url] Encoded info_hash: {}", percent_encode(&hex::decode(context.info_hash.clone())?, &URL_ENCODE_RESERVED).to_string());
    let info_hash = percent_encode(&hex::decode(context.info_hash.clone())?, &URL_ENCODE_RESERVED).to_string();
    let peer_id = PEER_ID.get().ok_or("Failed to get peer id, is it set ?")?.clone();
    let peer_id = percent_encode(&peer_id, &URL_ENCODE_RESERVED).to_string();

    let query = vec![
        ("port", "6881".to_string()),
        ("uploaded", "0".to_string()),
        ("downloaded", "0".to_string()),
        ("left", context.length.to_string()),
        ("corrupt", "0".to_string()),
        ("event", "started".to_string()),
        ("numwant", "200".to_string()),
        ("compact", "1".to_string()),
        ("no_peer_id", "1".to_string()),
    ];

    let url = format!("{}?info_hash={}&peer_id={}", context.announce, info_hash, peer_id);

    Ok((url, query))
}

pub async fn request_peers(context: &EngineContext) -> SyncResult<Vec<Peer>> {
    let url = build_tracker_url(context)?;
    println!("[request_peers] Requesting peers from tracker: {}", &url.0);

    let response = Client::new().get(&url.0).query(&url.1).send().await?.error_for_status()?;
    println!("[request_peers] Received response from tracker");
    println!("[request_peers] Response code: {}", response.status());

    let response = response.bytes().await?;
    let parsed = serde_bencode::from_bytes::<TrackerResponse>(&response)?;

    let peers = parsed.peers.ok_or("Missing peers in tracker response")?;
    Peer::from_bytes(peers.as_ref())
}