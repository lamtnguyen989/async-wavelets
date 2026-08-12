mod audio;
mod cwt;

use axum::extract::{DefaultBodyLimit, Multipart};


const MAX_UPLOAD_BYTES: usize = 40 * 1024 * 1024;  // 40 MB
const PORT: &str = "7777";

#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    // Making sure we can embed the Python interpreter
    tracing::info!("initializing embedded Python interpreter...");
    pyo3::Python::attach(|py| {
        let version = pyo3::Python::version_str();
        tracing::info!(python_version = %version, "embedded Python interpreter ready!");
    });
    
    Ok(())
}
