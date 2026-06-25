mod wavelet;

use anyhow::{Result};
use std::path::{PathBuf};
use clap::{Parser};

use wavelet::*;

/// CLI Args
#[derive(Parser, Debug)]
struct Args
{
    // Data path 
    #[arg(short, long, default_value="data")]
    data: PathBuf,

    // Beta parameters
    #[arg(short, long, value_delimiter=',')]
    beta: Vec<f64>,

    // Gamma parameters
    #[arg(short, long, value_delimiter=',')]
    gamma: Vec<f64>,

    // Min frequency considered for scalogram (in Hz)
    #[arg(long, default_value_t=20.0)]
    freq_min: f32,

    // Max frequency considered for scalogram (in Hz)
    #[arg(long, default_value_t=10000.0)]
    freq_max: f32,
}

async fn say_hello() -> () {
    println!("Hello, world!");
}


/// Entry point
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    say_hello().await;
    
    Ok(())
}
