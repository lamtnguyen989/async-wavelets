use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::todo;

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

pub struct DecodedAudio
{
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError 
{
    #[error("Could not recognize this as a supported audio format (MP3, FLAC, or WAV): {0}")]
    UnrecognizedFormat(String),
    #[error("No playable audio track found in file")]
    NoAudioTrack,
    #[error("Unsupported audio codec: {0}")]
    UnsupportedCodec(String),
    #[error("sample rate could not be determined")]
    UnknownSampleRate,
    #[error("Demuxing error: {0}")]
    Demux(String),
    #[error("Decoding error: {0}")]
    Decode(String),
    #[error("No audio samples in the decoding")]
    EmptyAudio,
}

pub fn decode_audio(bytes: Vec<u8>) -> Result<DecodedAudio, DecodeError>
{
    todo!();
}