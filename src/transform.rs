use crate::wavelet::*;
use crate::source::*;

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

/// Wave coefficient type (i.e. the type of values that will be computed for scalograms)
#[derive(Debug, Clone)]
pub enum WaveCoefficient
{
    Magnitude,  // |W(t,s)|
    Power,      // |W(t,s)|^2
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
    pub coeff_type:         WaveCoefficient,

    // Index and offsets
    pub scalogram_index:    usize,
    pub time_offset:        usize,

    // Data
    pub rows:               Vec<GmwScaleRow>,
}

impl GmwScalogram
{

}

pub async fn gmw_cwt() {
    todo!();
}