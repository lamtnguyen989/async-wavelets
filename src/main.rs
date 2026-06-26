mod wavelet;
mod source;
mod transform;

use anyhow::{Result};
use std::path::{PathBuf};
use clap::{Parser};

use wavelet::*;
use source::*;
use transform::*;

/// CLI Args
#[derive(Parser, Debug)]
struct Args
{
    // Data path 
    #[arg(short, long, default_value="data")]
    data: PathBuf,

    // Beta parameters
    #[arg(short, long, value_delimiter=',', default_value="3")]
    beta: Vec<f64>,

    // Gamma parameters
    #[arg(short, long, value_delimiter=',', default_value="3")]
    gamma: Vec<f64>,

    // Normalization metric
    #[arg(short, long, default_value="L1")]
    metric: String,

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

    // Processing CLI args data
    let args = Args::parse();
    let norm_metric = match args.metric.as_str() {
        "L1" => Normalization::L1,
        "L2" => Normalization::L2,
        _ => {println!("Unknown metric context, default to L1"); Normalization::L1}
    };

    let family: Vec<GeneralizedMorseWavelet> = args
        .beta.iter().zip(args.gamma.iter())
        .map(|(&b, &g)| {GeneralizedMorseWavelet::new((b, g), norm_metric)})
        .collect();

    // Compute the scalograms 



    say_hello().await;
    
    Ok(())
}
