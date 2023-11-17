#[derive(Debug, thiserror::Error)]
pub enum K8sCSIXetFSError {
    #[error("{0}")]
    GenericError(String),
    #[error("IOError {0}")]
    IOError(#[from] std::io::Error),
    #[error("TransportError {0}")]
    TonicTransportError(#[from] tonic::transport::Error),
}

impl From<K8sCSIXetFSError> for String {
    fn from(value: K8sCSIXetFSError) -> Self {
        format!("{value}")
    }
}