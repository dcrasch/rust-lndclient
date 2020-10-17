fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/")
        .build_server(false)
        .compile(&["protos/lnrpc/rpc.proto"], &["protos/"])?;
    Ok(())
}
