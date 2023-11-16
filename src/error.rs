#[derive(Debug, thiserror::Error)]
pub enum K8sCSIXetFSError {
    #[error("IOError {0}")]
    IOError(#[from] std::io::Error),
    #[error("TransportError {0}")]
    TonicTransportError(#[from] tonic::transport::Error),
}
