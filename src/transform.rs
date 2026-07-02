use crate::wavelet::*;
use crate::source::*;

use std::sync::Arc;
use rustfft::{Fft, FftPlanner};

/// Configuration for running the Wavelet Transform pipeline
/// The config is mainly to address memory usage issues nature of CWT in general
#[derive(Copy, Clone, Debug)]
pub struct TransformConfig
{
    pub block_size: usize,
    pub overlap_size: usize,
    pub n_cpus: usize,  // Number of actual concurrency processors for number crunching
}

impl TransformConfig
{
    /// Constructor
    pub fn new(block_size: usize, overlap_size: usize) -> Self {
        return Self {
            block_size: block_size,
            overlap_size: overlap_size,
            n_cpus: std::thread::available_parallelism()
                        .map(|t| t.get())
                        .unwrap_or(4),
        }
    }
}

/// Scalogram Row for Generalized Morse Wavelet
#[derive(Debug, Clone)]
pub struct GmwScaleRow
{
    pub scale_index:    f64,
    pub scale:          f64,
    pub frequency:      f64,
    pub wave_coeff:     Vec<f64>,
}

impl GmwScaleRow
{

}


/// Generalized Morse Wavelet Scalogram data container
#[derive(Debug, Clone)]
pub struct GmwScalogram
{
    // Metadata
    pub name:               String,
    pub params:             GmwParams,
    pub sample_rate:        u32,
    pub overlap_size:       usize,
    pub coeff_type:         WaveCoefficientType,

    // Index and offsets
    pub scalogram_index:    usize,
    pub time_offset:        usize,

    // Data
    pub rows:               Vec<GmwScaleRow>,
}

impl GmwScalogram
{

}


/// Convolution Filter for the Generalized Morse Wavelet
/// Eventhough memory footprint expensive, it is build only once (lookup table)
pub struct GmwFilter
{
    // Metadata to build filter
    fft_size: usize,
    overlap_size: usize,
    sample_rate: u32,
    period: f64,

    // Data containers
    scales: Vec<f64>,
    frequencies: Vec<f64>,

    // FFT plans
    fft_fwd: Arc<dyn Fft<f64>>,
    fft_inv: Arc<dyn Fft<f64>>,
}

impl GmwFilter
{
    pub fn new(
        wavelet: &GeneralizedMorseWavelet,
        freq_min: f64,
        freq_max: f64,
        sample_rate: f64,
        fft_size: usize,
        n_scales: usize,
    ) -> Self {
        let peak_freq = wavelet.params.peak_freq();

        todo!("Implement the constructor!");
    }
}
