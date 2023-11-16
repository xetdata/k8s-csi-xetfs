use std::path::Path;
use tokio::net::UnixListener;
use k8s_csi_xetfs::node::XetHubCSIService;
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
    XetHubCSIService::new();

    let listener = UnixListener::bind(Path::new(UNIX_SOCKET_PATH.as_str()))?;
    let stream = UnixListenerStream::new(listener);

    let xethub_service = XetHubCSIService::new();

    let node_server = k8s_csi_xetfs::proto::csi::v1::node_server::NodeServer::new(xethub_service);
    let identity_server = k8s_csi_xetfs::proto::csi::v1::identity_server::IdentityServer::new(IdentityService);

    Server::builder()
        .add_service(node_server)
        .add_service(identity_server)
        .serve_with_incoming(stream)
        .await
        .map_err(K8sCSIXetFSError::TonicTransportError)
}
