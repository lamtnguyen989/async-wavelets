use std::net::SocketAddr;

use crate::pb::{processing_service_server::ProcessingServiceServer};
use crate::cwt::WaveletServer;

use tokio::net::TcpListener;
use tonic::service::Routes as TonicRoutes;
use tonic_web::GrpcWebLayer;
use tower_http::services::ServeDir;

mod audio;
mod cwt;

pub mod pb {
    tonic::include_proto!("wavelet");
}

const MAX_GRPC_MESSAGE_MB: usize = 16;  // Default is 4 MB so needs to pump those rookie numbers up
const MAX_GRPC_MESSAGE_BYTES: usize = MAX_GRPC_MESSAGE_MB * 1024 * 1024;
const ADDR: &str = "0.0.0.0:10000";
const FRONTEND_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend");
const DEFAULT_PYTHON_SRC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), ".");   // For future-proofing of layout 

#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    // Setting up tracer
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    // Making sure we can embed the Python interpreter
    tracing::info!("Initializing embedded Python interpreter...");
    pyo3::Python::attach(|_py| {
        let version = pyo3::Python::version_str();
        tracing::info!(python_version = %version, "embedded Python interpreter ready!");
    });

    // Importing the wavelet transform module
    let python_src_dir = std::env::var("PYTHON_SRC_DIR").unwrap_or_else(|_| DEFAULT_PYTHON_SRC_DIR.to_string());
    cwt::init_wavelet_python_module(&python_src_dir)?;
    tracing::info!("Wavelet module imported success!");

    // Initializing scalogram processing service
    let wavelet_service = ProcessingServiceServer::new(WaveletServer)
        .max_decoding_message_size(MAX_GRPC_MESSAGE_BYTES)
        .max_encoding_message_size(MAX_GRPC_MESSAGE_BYTES);

    // Routing gRPC services for requests that are grpc-web
    let mut tonic_routes = TonicRoutes::builder();
    tonic_routes.add_service(wavelet_service);

    // Main web-level router
    let app = tonic_routes.routes()
            .into_axum_router()
            .layer(GrpcWebLayer::new())
            .fallback_service(ServeDir::new(FRONTEND_DIR));

    // Exposing service
    let addr: SocketAddr = ADDR.parse()?;
    tracing::info!("Listening on http://{addr}");
    tracing::info!("Serving static files from {FRONTEND_DIR}");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
