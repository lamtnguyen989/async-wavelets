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