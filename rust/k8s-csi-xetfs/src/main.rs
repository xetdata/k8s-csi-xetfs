use std::path::Path;
use clap::Parser;
use tokio::net::UnixListener;
use k8s_csi_xetfs::node::XetHubCSIDriver;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tracing::info;
use k8s_csi_xetfs::args::DriverArgs;
use k8s_csi_xetfs::identity::IdentityService;
use k8s_csi_xetfs::error::K8sCSIXetFSError;
use k8s_csi_xetfs::initialize_tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), K8sCSIXetFSError> {
    initialize_tracing_subscriber()?;

    let DriverArgs {
        node_id, endpoint
    } = DriverArgs::parse();
    info!("starting driver with node id: {node_id} endpoint: {endpoint}");

    // remove socket file if present, ignore error if DNE
    let _ = std::fs::remove_file(endpoint.as_str());
    let listener = UnixListener::bind(Path::new(endpoint.as_str()))?;
    let stream = UnixListenerStream::new(listener);

    let driver = XetHubCSIDriver::new(node_id);
    let node_server = k8s_csi_xetfs::proto::csi::v1::node_server::NodeServer::new(driver);
    let identity_server = k8s_csi_xetfs::proto::csi::v1::identity_server::IdentityServer::new(IdentityService);

    info!("starting service");
    Server::builder()
        .add_service(node_server)
        .add_service(identity_server)
        .serve_with_incoming(stream)
        .await
        .map_err(K8sCSIXetFSError::TonicTransportError)
}
