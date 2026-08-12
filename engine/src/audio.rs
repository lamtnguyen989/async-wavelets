use anyhow::{Result, Context};
use std::path::{Path, PathBuf};

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