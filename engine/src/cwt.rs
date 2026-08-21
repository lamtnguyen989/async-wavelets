use anyhow::{Result, Context};
use pyo3::prelude::*;
use pyo3::types::{PyDict};
use numpy::PyArray1;

use tonic::{Request, Response, Status};

use crate::audio::{decode_audio, DecodeError};
use crate::pb::{AudioUploadRequest, WaveletResult};
use crate::pb::processing_service_server::ProcessingService;

/// File upload limit
const MAX_UPLOAD_MB: usize = 5;
const MAX_UPLOAD_BYTES: usize = MAX_UPLOAD_MB * 1024 * 1024;

/// Struct storing scalogram result image
pub struct ScalogramResult {
    pub image: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Initalize Python module 
pub fn init_wavelet_python_module(python_src_dir: &str) -> Result<()>
{
    let py_init = Python::attach(|py| -> Result<()> {
        // Check if the Python source directory is already in use, if not append
        let sys = py.import("sys")?;
        let sys_path = sys.getattr("path")?;
        let dir_query = sys_path.call_method1("__contains__", (python_src_dir,))?;
        let dir_present: bool = dir_query.extract()?;
        if !dir_present {
            sys_path.call_method1("insert", (0, python_src_dir))?;
        }

        // Import the Morse Wavelet module (probably need to incorporate more wavelets in the future)
        let _ = Python::import(py, "morse_wavelet")
                    .context("Importing wavelet transform module");
        
        Ok(())
    });
    
    return py_init;
}


/// Plotting the scalogram with JAX from the Python script
pub fn compute_scalogram(
    audio_signal: &[f32], 
    sr: u32, 
    f_min: f32, 
    f_max: f32,
    n_scales: u32, 
    beta: f32, 
    gamma: f32
) -> Result<ScalogramResult>
{
    return Python::attach(|py| -> Result<ScalogramResult> {
        // Setting up the scalogram plotting call
        let morse_wavelet = PyModule::import(py, "morse_wavelet").context("importing morse_wavelet")?;
        let input_signal = PyArray1::from_slice(py, audio_signal);
        let kwargs = PyDict::new(py);
        kwargs.set_item("f_min", f_min)?;
        kwargs.set_item("f_max", f_max)?;
        kwargs.set_item("n_scales", n_scales)?;
        kwargs.set_item("beta", beta)?;
        kwargs.set_item("gamma", gamma)?;
        kwargs.set_item("title", "Audio Scalogram")?;
        kwargs.set_item("return_bytes", true)?;
        
        let result = morse_wavelet.getattr("scalogram")?
            .call((input_signal, sr), Some(&kwargs))
            .context("Computing scalogram")?;
        
        let (img, wt, ht): (Vec<u8>, u32, u32) = result.extract().context("Extracting result")?;

        return Ok(ScalogramResult {
            image: img,
            width: wt,
            height: ht
        });
    });
}

pub struct WaveletServer;

#[tonic::async_trait]
impl ProcessingService for WaveletServer {
    async fn process_audio(&self, request: Request<AudioUploadRequest>) -> Result<Response<WaveletResult>, Status> {
        // Pre-processing incoming audio data
        let req = request.into_inner();

        if req.audio_data.is_empty() {
            return Err(Status::invalid_argument("No audio data recieved!"));
        }

        if req.audio_data.len() > MAX_UPLOAD_BYTES {
            return Err(Status::invalid_argument(
                format!("Audio file too large! Max size is {} bytes, recieved {} bytes", MAX_UPLOAD_BYTES, req.audio_data.len())
            ));
        }

        // Decode the audio
        let decoded_audio = tokio::task::spawn_blocking(move || decode_audio(req.audio_data)).await
            .map_err(|e| Status::internal(format!("Audio decoding task panicked: {e}")))?
            .map_err(|err| {
                match err {
                    DecodeError::UnrecognizedFormat(_) | DecodeError::UnsupportedCodec(_)   => Status::invalid_argument(err.to_string()),
                    DecodeError::NoAudioTrack | DecodeError::EmptyAudio                     => Status::invalid_argument(err.to_string()),
                    DecodeError::UnknownSampleRate | DecodeError::Demux(_) | DecodeError::Decode(_) => Status::internal(err.to_string()),
                }
            })?;
        
        // Pulling metadata from decoded audio
        let n_samples = decoded_audio.samples.len() as u32;
        let sr = decoded_audio.sample_rate;
        let codec = decoded_audio.codec.to_string();
        
        // Launching the JAX compute task
        let f_max = (sr as f32 / 2.0).min(15000.0);
        let f_min = 20.0;
        let n_scales = 64;
        let (beta, gamma) = (3.0, 20.0);

        let compute_result = tokio::task::spawn_blocking(move || {
            compute_scalogram(&decoded_audio.samples, sr, f_min, f_max, n_scales, beta, gamma)
        }).await
        .map_err(|e| Status::internal(format!("Scalogram task panicked: {e}")))?
        .map_err(|e| Status::internal(format!("Scalogram computation failed: {e:#}")))?;

        return Ok(Response::new(WaveletResult {
            image:          compute_result.image,
            sample_rate:    sr,
            n_samples:      n_samples,
            codec:          codec,
            width:          compute_result.width,
            height:         compute_result.height,
            error:          String::new(),
        }));
    }
}