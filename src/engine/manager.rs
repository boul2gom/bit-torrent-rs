use std::path::PathBuf;
use crate::engine::context::EngineContext;
use crate::engine::Engine;
use crate::shared::{PEER_ID, SyncResult};
use crate::types::bencode::MetaInfoFile;

pub struct EngineManager {
    pub engines: Vec<Engine>,
}

impl EngineManager {
    pub async fn new(meta_info: PathBuf, destination: PathBuf) -> SyncResult<Self> {
        let meta_info = MetaInfoFile::from_file(meta_info).await?;
        let mut engines = Vec::new();

        PEER_ID.set(b"-RS0001-NULLPTR-0000".clone()).unwrap();

        if meta_info.is_single_file_mode() {
            println!("[EngineManager - new] Single file mode");
            let length = meta_info.info.length.ok_or("Missing length in .torrent file")?;
            let destination = destination.join(meta_info.info.name.clone());
            println!("[EngineManager - new] Destination: {}", destination.display());
            //Remove after last separator because it's a file
            let parent = destination.parent().ok_or("Missing parent directory")?;
            tokio::fs::create_dir_all(parent).await?;
            println!("[EngineManager - new] Created directory");
            tokio::fs::File::create(&destination).await?;
            println!("[EngineManager - new] Created file");

            let context = EngineContext::new(meta_info.clone(), length, destination.clone()).await?;
            println!("[EngineManager - new] Created context");

            let engine = Engine::new(context);
            println!("[EngineManager - new] Created engine");
            engines.push(engine);
        }

        if meta_info.is_multi_file_mode() {
            println!("[EngineManager - new] Multi file mode");
            let files = meta_info.info.files.as_ref().ok_or("Missing files in .torrent file")?;

            for file in files {
                let length = file.length.ok_or("Missing length in .torrent file")?;
                let path_vec = file.path.as_ref().ok_or("Missing path in .torrent file")?;

                let mut destination = destination.clone();
                path_vec.iter().for_each(|path| destination = destination.join(path));
                println!("[EngineManager - new] Destination: {}", destination.display());
                let parent = destination.parent().ok_or("Missing parent directory")?;
                tokio::fs::create_dir_all(parent).await?;
                println!("[EngineManager - new] Created directory");
                tokio::fs::File::create(&destination).await?;
                println!("[EngineManager - new] Created file");

                let context = EngineContext::new(meta_info.clone(), length, destination.clone()).await?;
                println!("[EngineManager - new] Created context");

                let engine = Engine::new(context);
                println!("[EngineManager - new] Created engine");
                engines.push(engine);
            }
        }

        Ok(Self {
            engines
        })
    }

    pub fn using_single_mode(&self) -> bool {
        self.engines.len() == 1
    }

    pub fn using_multi_mode(&self) -> bool {
        self.engines.len() > 1
    }

    pub async fn start_engines(&mut self) -> SyncResult<()> {
        let mut index = 0;
        for engine in self.engines.iter_mut() {
            println!("[EngineManager - start_engines] Starting engine {}", index);
            engine.download_torrent().await?;
            index += 1;
        }

        Ok(())
    }
}