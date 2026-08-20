use anyhow::{Context, Result};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::{hint, todo};

use symphonia::core::errors::Error::ResetRequired;
use symphonia::core::audio::GenericAudioBufferRef;
use symphonia::core::codecs::audio::{AudioDecoder, AudioDecoderOptions, CODEC_ID_NULL_AUDIO};
use symphonia::core::codecs::registry::CodecRegistry;
use symphonia::core::codecs::CodecParameters;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::{Hint, Probe, ProbeOptions};
use symphonia::core::formats::{FormatOptions, FormatReader, TrackType};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::packet::Packet;

pub struct DecodedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub codec: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
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

pub fn decode_audio(bytes: Vec<u8>) -> Result<DecodedAudio, DecodeError> {
    // Setting Media Source stream on the stream of bytes
    let cursor = Cursor::new(bytes);
    let mss_options = MediaSourceStreamOptions::default();
    let mss = MediaSourceStream::new(Box::new(cursor), mss_options);

    // Probing the input
    let mut probe = Probe::default();
    let hint = Hint::new();
    let meta_opts = MetadataOptions::default();
    let format_opts = FormatOptions::default();
    let mut format = probe
        .probe(&hint, mss, format_opts, meta_opts)
        .map_err(|err| DecodeError::UnrecognizedFormat(err.to_string()))?;

    // Check if the bytes input is an audio source
    let track = format
        .first_track_known_codec(TrackType::Audio)
        .ok_or(DecodeError::NoAudioTrack)?;

    // Extracting audio parameters
    let audio_params = match &track.codec_params {
        Some(CodecParameters::Audio(params)) => params.clone(),
        _ => return Err(DecodeError::NoAudioTrack),
    };

    let sample_rate = audio_params
        .sample_rate
        .ok_or(DecodeError::UnknownSampleRate)?;
    let n_channels = audio_params
        .channels
        .as_ref()
        .map(|c| c.count() as u16)
        .unwrap_or(1);

    // Initialize the codec (decoder)
    let codec_registry = CodecRegistry::new();
    let decoder_opts = AudioDecoderOptions::default();
    let mut decoder = codec_registry
        .make_audio_decoder(&audio_params, &decoder_opts)
        .map_err(|err| DecodeError::UnsupportedCodec(err.to_string()))?;

    let detected_codec = decoder.codec_info().short_name;

    // Sampling the audio (note we are squashing multiple channel to a mono channel by averaging)
    let mut mono_samples: Vec<f32> = vec![];
    let mut interleaved_buffer: Vec<f32> = vec![];
    let track_id = track.id;
    loop {
        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            Err(SymphoniaError::ResetRequired) => break,
            Err(e) => return Err(DecodeError::Demux(e.to_string())),
        };

        if packet.track_id != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                decoded.copy_to_vec_interleaved(&mut interleaved_buffer);
                append_mono_dowmixed(&interleaved_buffer, n_channels, &mut mono_samples);
            }
            Err(SymphoniaError::DecodeError(_)) => continue, // Currently skip corrupt/undecodable packets
            Err(e) => return Err(DecodeError::Decode(e.to_string())),
        }
    }

    if mono_samples.is_empty() {
        return Err(DecodeError::EmptyAudio);
    }

    return Ok(DecodedAudio {
        samples: mono_samples,
        sample_rate: sample_rate,
        channels: n_channels,
        codec: detected_codec,
    });
}

fn append_mono_dowmixed(interleaved_buff: &[f32], channels: u16, out: &mut Vec<f32>) {
    match channels {
        1 => out.extend_from_slice(interleaved_buff),
        _ => {
            for frame in interleaved_buff.chunks_exact(channels as usize) {
                let avg_of_packet = frame.iter().sum::<f32>() / channels as f32;
                out.push(avg_of_packet);
            }
        }
    }
}
