use anyhow::{Result, Context};
use pyo3::prelude::*;
use pyo3::types::{PyDict};
use numpy::PyArray1;


/// Struct storing scalogram result data in Row-major layout: [n_scales x n_time]
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
            sys_path.call_method1("insert", (python_src_dir, ))?;
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

