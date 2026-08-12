use std::sync::OnceLock;
use std::ffi::CString;
use std::todo;

use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::ffi::c_str;

const WAVELET_SOURCE: &str = include_str!("morse_wavelet.py");
static CWT_MODULE: OnceLock<Py<PyModule>> = OnceLock::new();

/// Wavelet Analysis Parameters
#[derive(Debug, Clone, Copy)]
pub struct AnalysisParams 
{
    pub n_scales: u32,
    pub f_min: f32,
    pub f_max: f32,
    pub beta: f32,
    pub gamma: f32,
    pub max_time_bins: u32,
}

impl Default for AnalysisParams {
    fn default() -> Self {
        return Self {
            n_scales:       64,
            f_min:          20.0,
            f_max:          15000.0,
            beta:           3.0,
            gamma:          20.0,
            max_time_bins:  1500,
        }
    }
}

/// Struct storing scalogram result data in Row-major layout: [n_scales x n_time]
pub struct ScalogramResult {
    pub magnitude: Vec<f32>,
    pub n_scales: usize,
    pub n_time: usize,
    pub frequencies_hz: Vec<f32>,
    pub time_seconds: Vec<f32>,
    pub max_magnitude: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("python error: {0}")]
    Python(String),
}

impl From<PyErr> for EngineError {
    fn from(e: PyErr) -> Self {
        EngineError::Python(e.to_string())
    }
}

/// Loading JAX Wavelet python source as a module
fn wavelet_module(py: Python<'_>) -> PyResult<&Bound<'_, PyModule>>
{
    let source = CString::new(WAVELET_SOURCE).expect("Fail to read the Python wavelet source code!");

    let module = CWT_MODULE.get_or_init(|| {
        PyModule::from_code(py, &source, c_str!("morse_wavelet.py"), c_str!("cwt"))
            .expect("Fail to load JAX source as an embedded Python module")
            .unbind()
    });

    return Ok(module.bind(py));
}

pub fn compute_scalogram() 
{
    todo!();
}