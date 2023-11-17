use std::path::Path;
use tokio::net::UnixListener;
use k8s_csi_xetfs::node::XetHubCSIDriver;
use once_cell::sync::Lazy;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use k8s_csi_xetfs::constants::*;
use k8s_csi_xetfs::identity::IdentityService;
use k8s_csi_xetfs::error::K8sCSIXetFSError;

static UNIX_SOCKET_PATH: Lazy<String> = Lazy::new(|| {
    std::env::var(UNIX_SOCKET_PATH_CSI_ENV_VAR).unwrap_or_default()
});

#[tokio::main]
async fn main() -> Result<(), K8sCSIXetFSError> {
    let node_id = String::from("TODO");

    let listener = UnixListener::bind(Path::new(UNIX_SOCKET_PATH.as_str()))?;
    let stream = UnixListenerStream::new(listener);


    let driver = XetHubCSIDriver::new(node_id);
    let node_server = k8s_csi_xetfs::proto::csi::v1::node_server::NodeServer::new(driver);
    let identity_server = k8s_csi_xetfs::proto::csi::v1::identity_server::IdentityServer::new(IdentityService);

    Server::builder()
        .add_service(node_server)
        .add_service(identity_server)
        .serve_with_incoming(stream)
        .await
        .map_err(K8sCSIXetFSError::TonicTransportError)
}
