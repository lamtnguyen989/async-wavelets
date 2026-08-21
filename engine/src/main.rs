mod audio;
mod cwt;

use axum::extract::{DefaultBodyLimit, Multipart};


const MAX_UPLOAD_BYTES: usize = 5 * 1024 * 1024;  // 5 MB
const MAX_GRPC_MESSAGE_BYTES: usize = 16 * 1024 * 1024; // 16
const PORT: &str = "7777";

#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    // Setting up tracer
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
