fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["protos/csi.proto"], &["protos/"])?;

    Ok(())
}
