use tokio::sync::Mutex;
use std::collections::HashMap;
use tonic::Status;
use tracing::warn;
use crate::driver::mount::{mount, unmount};
use crate::driver::volume::XetCSIVolume;

mod mount;
pub(crate) mod volume;

#[derive(Debug, Default)]
pub(crate) struct XetHubCSIDriver {
    // map of volume_id to XetCSIVolume
    // TODO: make this map be backed by file system for resilience
    volumes: Mutex<HashMap<String, XetCSIVolume>>,
}

impl XetHubCSIDriver {
    pub fn new() -> Self {
        XetHubCSIDriver {
            volumes: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) async fn publish(&self, volume_spec: XetCSIVolume) -> Result<(), Status> {
        let mut volumes = self.volumes.lock().await;
        if let Some(volume) = volumes.get(&volume_spec.volume_id) {
            return if volume_spec == *volume {
                // repeat request, already published
                Ok(())
            } else {
                // incompatible
                Err(Status::already_exists(format!("volume {} already exists", volume_spec.volume_id)))
            };
        }
        mount(&volume_spec).await.map_err(|e| {
            warn!("error in mount: {e:?}");
            Status::internal(e)
        })?;
        volumes.insert(volume_spec.volume_id.clone(), volume_spec);
        Ok(())
    }

    pub(crate) async fn unpublish(&self, volume_id: String) -> Result<(), Status> {
        let mut volumes = self.volumes.lock().await;
        let volume = volumes.get(volume_id.as_str()).ok_or(
           Status::not_found(format!("volume with volume id {volume_id} not found"))
        )?;
        unmount(volume.path.clone()).await.map_err(|e| {
            warn!("error in umount: {e:?}");
            Status::internal(e)
        })?;
        let _ = volumes.remove(volume_id.as_str());
        Ok(())
    }
}
