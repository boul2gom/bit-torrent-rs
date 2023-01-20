use std::path::PathBuf;
use tokio::runtime::Builder;
use bit_torrent_rs::engine::manager::EngineManager;

pub fn main() -> std::io::Result<()> {
    let worker_threads = std::env::var("WORKER_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap();

    let runtime = Builder::new_multi_thread()
        .thread_name("bit-torrent-rs-worker")
        .worker_threads(worker_threads)
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async_bootstrap())
}

pub async fn async_bootstrap() -> std::io::Result<()> {
    let path = PathBuf::from("examples/The Matrix 4 - Resurrections.torrent");
    let destination = PathBuf::from("The Matrix 4 - Resurrections");

    let mut manager = EngineManager::new(path, destination).await.expect("Failed to create engine manager");
    manager.start_engines().await.expect("Failed to start engines");

    Ok(())
}
