fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    tonic_prost_build::configure()
        .build_server(true) // At the time of writing this is a same machine service
        .build_client(false)
        .compile_protos(&["../proto/wavelet.proto"], &["../proto"])?;

    Ok(())
}