use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tokio_stream::wrappers::{ReadDirStream};
use tokio_stream::{StreamExt};
use symphonia::core::codecs::{
    CodecParameters,
    audio::{AudioDecoder, AudioDecoderOptions},
};

/// Finding audio files with a specified directory (non-recursive)
pub async fn fetch_audio_files(dir: &Path) -> Result<Vec<PathBuf>> {
    // Initialize file paths container
    let mut file_paths: Vec<PathBuf> = vec![];
    
    // Creating directory stream
    let read_dir = tokio::fs::read_dir(dir).await
                        .with_context(|| format!("Can not read from: {}", dir.display()))?;
    let mut stream = ReadDirStream::new(read_dir);

    // Accumulate audio file paths (based on extension cuz idk of a better way atm lol)
    while let Some(entry) = stream.next().await {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                if matches!(ext.to_lowercase().as_str(), "flac" | "wav" | "mp3") {
                    file_paths.push(entry_path);
                }
            }
        }
    }

    Ok(file_paths)
}

/// Signal source metadata
#[derive(Debug, Clone)]
pub struct SignalInfo
{
    pub name:           String,
    pub path:           Option<PathBuf>,
    pub sample_rate:    u32,
    pub n_samples_hint: Option<usize>,  // Total sample size if known
}


/// Chunk of audio as f32 samples
pub struct AudioStream
{
    pub info: SignalInfo,
    decoder: Box<dyn AudioDecoder>,
}

impl AudioStream
{

}
