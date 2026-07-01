use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tokio_stream::wrappers::{ReadDirStream};
use tokio_stream::{StreamExt};

use symphonia::core::codecs::{CodecParameters};
use symphonia::core::codecs::registry::{CodecRegistry};
use symphonia::core::codecs::audio::{AudioDecoder, AudioDecoderOptions, CODEC_ID_NULL_AUDIO};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::formats::{FormatReader, FormatOptions};
use symphonia::core::formats::probe::{Hint, Probe, ProbeOptions};
use symphonia::core::meta::{MetadataOptions};
use symphonia::core::errors::{Error as SymphoniaError};
use symphonia::core::audio::{GenericAudioBufferRef};
use symphonia::core::packet::{Packet};

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
    name:           String,
    path:           Option<PathBuf>,
    sample_rate:    u32,
    n_samples:      u64,
    n_channels:     u16,
}

impl SignalInfo
{
    // Metadata getters
    pub fn get_name(&self) -> &str {return &self.name;}
    pub fn get_path(&self) -> Option<&Path> {return self.path.as_deref();}
    pub fn get_sample_rate(&self) -> u32 {return self.sample_rate;}
    pub fn get_n_samples(&self) -> u64 {return self.n_samples;}
    pub fn get_n_channels(&self) -> u16 {return self.n_channels;}
}


/// Audio Stream from audio files
pub struct AudioStream
{
    pub info:   SignalInfo,
    decoder:    Box<dyn AudioDecoder>,
    format:     Box<dyn FormatReader>,
    track_id:   u32,
}

impl AudioStream
{
    /// Start the audio stream from a file 
    /// Most audio files needs to be decoded which is compute-heavy so sadly this needs to be synchronous :(
    pub fn open(path: &Path) -> Result<Self> {
        // Open file from path
        let file = std::fs::File::open(path)
                        .with_context(|| format!("Can not open {}", path.display()))?;
        
        // Turning opened file into an input stream
        let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

        // Adding file extension hint
        let mut hint = Hint::new();
        if let Some(extension) = path.extension().and_then(|ext| {ext.to_str()}) {
            hint.with_extension(extension);
        }

        // Probing the audio file
        let probe = Probe::new_with_options(&ProbeOptions::default());
        let format: Box<dyn FormatReader> = probe.probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
                                                .with_context(|| {format!("Can not probe {}", path.display())})?;

        // Find audio track from the format probe
        let track = format.tracks().iter().find({
            |t| {match &t.codec_params {
                Some(CodecParameters::Audio(p)) => p.codec != CODEC_ID_NULL_AUDIO,    // Skipping sentinel audio codec ID
                _ => false
            }}
        }).context("No audio tracks found")?;
        
        // Audio parameters  of the track
        let audio_params = match &track.codec_params {
            Some(CodecParameters::Audio(p)) => p,
            _ => anyhow::bail!("Track is not an audio track"),
        };
        
        // Extract audio metadata from track
        let id: u32 = track.id;
        let sample_rate = audio_params.sample_rate.context("Unknown sample rate!")?;
        let n_frames = track.num_frames.context("Unknown track size!")?;
        let name = path.file_stem().and_then(|s| s.to_str())
                        .unwrap_or("Unknown file name").to_string();
        let n_channels = audio_params.channels.as_ref()
                                        .map(|c| c.count())
                                        .unwrap_or(1);

        // Decoder object
        let audio_decoder: Box<dyn AudioDecoder> = CodecRegistry::new()
                                                    .make_audio_decoder(audio_params, &AudioDecoderOptions::default())
                                                    .context("Fail to create audio decoder")?;

        return Ok(Self {
            info: SignalInfo {
                name:           name,
                path:           Some(path.to_path_buf()),
                sample_rate:    sample_rate,
                n_samples:      n_frames,
                n_channels:     n_channels as u16,
            },
            decoder:    audio_decoder,
            format:     format,
            track_id:   id
        });
    }

    /// Next data chunk from audio stream (as a collection of f32's, though subject to change for better usage)
    pub fn next(&mut self) -> Result<Option<Vec<f32>>> {
        loop {
            // Grab the next packet
            let packet: Packet = match self.format.next_packet() {
                Ok(p) => p.unwrap(),
                Err(SymphoniaError::ResetRequired) => {self.decoder.reset(); continue;},
                Err(e) => {return Err(e.into());}
            };
            if packet.track_id != self.track_id {continue;}

            // Decode the packet 
            let decoded_packet = self.decoder.decode(&packet)?;
            let samples: Vec<f32> = match decoded_packet {
                _ => {todo!();}
            };

            return Ok(Some(samples));
        }
    }
}

/// Iterator for implementing Overlap-Save fast convolution
pub struct OverlapSaveIter
{

}

impl OverlapSaveIter
{

}
