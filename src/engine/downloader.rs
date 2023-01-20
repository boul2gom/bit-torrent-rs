use std::time::Duration;
use async_channel::{Receiver, Sender};
use tokio::io::AsyncWriteExt;
use tokio::time;
use crate::connection::client::Client;
use crate::shared::{MAX_BACKLOG, MAX_BLOCK_SIZE, SyncResult};
use crate::types::peer::Peer;
use crate::types::piece::{PieceProgress, PieceResult, PieceWork};

#[derive(Clone)]
pub struct Downloader {
    pub peer: Peer,
    pub info_hash: String,
    pub work_sender: Sender<PieceWork>,
    pub result_sender: Sender<PieceResult>,
    pub work_receiver: Receiver<PieceWork>,
}

impl Downloader {
    pub fn new(peer: Peer, info_hash: String, work_sender: Sender<PieceWork>, result_sender: Sender<PieceResult>, work_receiver: Receiver<PieceWork>) -> Self {
        Self {
            peer,
            info_hash,
            work_sender,
            result_sender,
            work_receiver
        }
    }

    pub async fn start_worker(&self) -> SyncResult<()> {
        println!("[Downloader - start_worker] Starting worker for peer {}:{}", self.peer.ip, self.peer.port);
        let mut client = Client::connect(self.peer.clone(), self.info_hash.clone()).await?;

        let result = self.start_safe_worker(&mut client).await;
        if result.is_err() {
            println!("[Downloader - start_worker] Shutting down worker");
            client.connection.shutdown().await?;
        }

        return result
    }

    pub async fn start_safe_worker(&self, client: &mut Client) -> SyncResult<()> {
        client.send_unchoke().await?;
        println!("[Downloader - start_safe_worker] Unchoke sent");
        client.send_interested().await?;
        println!("[Downloader - start_safe_worker] Interested sent");

        while let Ok(piece_work) = self.work_receiver.recv().await {
            if !client.bitfield.has_piece(piece_work.index as u32) {
                println!("[Downloader - start_safe_worker] Skipping piece {} because we don't have it", piece_work.index);
                self.work_sender.send(piece_work).await?;
                continue;
            }

            println!("[Downloader - start_safe_worker] Starting downloading piece {}", piece_work.index);
            let piece_data = piece_work.download_piece(client).await;
            if piece_data.is_err() {
                println!("[Downloader - start_safe_worker] Failed to download piece {}: {:?}", piece_work.index, piece_data.unwrap_err());
                self.work_sender.send(piece_work).await?;
                continue;
            }

            let piece_data = piece_data.unwrap();

            client.send_have(piece_work.index as u32).await?;
            let result = PieceResult {
                index: piece_work.index,
                data: piece_data,
            };

            self.result_sender.send(result).await?;
        }

        Ok(())
    }
}

impl PieceWork {
    pub async fn download_piece(&self, client: &mut Client) -> SyncResult<Vec<u8>> {
        let mut progress = PieceProgress::new(self.index);

        //30 seconds timeout to download a piece
        let timeout_duration = Duration::from_secs(30);
        let timeout = time::timeout(timeout_duration, self.download_piece_safe(client, &mut progress)).await;

        if timeout.is_err() {
            return Err("Timeout while downloading piece".into());
        }

        Ok(progress.data)
    }

    pub async fn download_piece_safe(&self, client: &mut Client, progress: &mut PieceProgress) -> SyncResult<()> {
        while progress.downloaded < self.length {
            if !client.choked {
                println!("[PieceWork - download_piece_safe] Client is not choked, requesting block for piece {}", self.index);
                while progress.backlog < MAX_BACKLOG && progress.requested < self.length {
                    let mut block_size = MAX_BLOCK_SIZE;

                    if self.length - progress.requested < block_size {
                        block_size = self.length - progress.requested;
                    }

                    client.send_request(self.index, progress.requested, block_size).await?;
                    println!("[PieceWork - download_piece_safe] Request sent for piece {} at offset {} with size {}", self.index, progress.requested, block_size);

                    progress.backlog += 1;
                    progress.requested += block_size;
                }
            }

            println!("[PieceWork - download_piece_safe] Waiting for block for piece {}", self.index);
            progress.parse_message(client).await?;
            println!("[PieceWork - download_piece_safe] Message parsed for piece {}", self.index);
        }

        Ok(())
    }
}