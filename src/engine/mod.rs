use std::io::SeekFrom;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use crate::engine::context::EngineContext;
use crate::engine::downloader::Downloader;
use crate::protocol::tracker;
use crate::shared::SyncResult;
use crate::types::piece::{PieceResult, PieceWork};
use crate::utils::data::calculator;

pub mod context;
pub mod manager;
pub mod downloader;

pub struct Engine {
    pub context: EngineContext,
    pub downloaders: Vec<Downloader>
}

impl Engine {
    pub fn new(context: EngineContext) -> Self {
        Self {
            context,
            downloaders: Vec::new()
        }
    }

    pub async fn download_torrent(&mut self) -> SyncResult<()> {
        println!("[Engine - download_torrent] Starting download");
        let (work_sender, work_receiver) = async_channel::bounded::<PieceWork>(self.context.pieces.len() + 1);
        let (result_sender, result_receiver) = async_channel::bounded::<PieceResult>(self.context.pieces.len() + 1);
        println!("[Engine - download_torrent] Created channels");

        for (index, hash) in self.context.pieces.iter().enumerate() {
            let length = calculator::calculate_piece_size(self.context.length, self.context.piece_length, index as u32);
            let piece_work = PieceWork::new(index as u32, hash.clone(), length);

            work_sender.send(piece_work).await?;
            println!("[Engine - download_torrent] Work sent for piece {}", index);
        }

        let peers = tracker::request_peers(&self.context).await?;
        println!("[Engine - download_torrent] Received peers");

        for peer in peers {
            let work_sender = work_sender.clone();
            let result_sender = result_sender.clone();
            let work_receiver = work_receiver.clone();

            let downloader = Downloader::new(peer, self.context.info_hash.clone(), work_sender, result_sender, work_receiver);
            self.downloaders.push(downloader);
        }
        println!("[Engine - download_torrent] Created downloaders");

        for downloader in self.downloaders.iter() {
            let downloader = downloader.clone();
            //let downloader = self.downloaders.get(0).unwrap().clone();
            let _ = tokio::spawn(async move {
                let result = downloader.start_worker().await;

                if result.is_err() {
                    println!("[Engine - download_torrent] Failed to start worker: {}", result.as_ref().unwrap_err());
                }

                return result
            }).await??;
        }
        println!("[Engine - download_torrent] Spawned downloaders");

        let mut downloaded_pieces = 0;

        while downloaded_pieces < self.context.pieces.len() {
            let piece_result = result_receiver.recv().await?;
            println!("[Engine - download_torrent] Received piece result for piece {}", piece_result.index);
            let (begin, _end) = calculator::calculate_bounds_for_piece(self.context.length, self.context.piece_length, piece_result.index);

            self.context.destination.seek(SeekFrom::Start(begin as u64)).await?;
            println!("[Engine - download_torrent] Seeking to {}", begin);
            self.context.destination.write_all(&piece_result.data).await?;
            println!("[Engine - download_torrent] Writing piece {}", piece_result.index);
            downloaded_pieces += 1;

            let percentage = (downloaded_pieces as f64 / self.context.pieces.len() as f64) * 100.0;
            println!("[Engine - download_torrent] Downloaded piece: {} of {} ({}%)", downloaded_pieces, self.context.pieces.len(), percentage);
        }

        work_sender.close();
        work_receiver.close();

        Ok(())
    }
}