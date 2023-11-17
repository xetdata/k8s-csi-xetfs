use crate::error::K8sCSIXetFSError;
use crate::node::XetCSIVolume;

pub(crate) async fn mount(volume_spec: &XetCSIVolume) -> Result<(), K8sCSIXetFSError> {
    todo!()
}

pub(crate) async fn unmount(path: String) -> Result<(), K8sCSIXetFSError> {
    todo!()
}
