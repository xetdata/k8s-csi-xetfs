use tokio::sync::Mutex;
use std::collections::HashMap;
use tonic::Status;
use tracing::warn;
use crate::driver::mount::{GitXetMounter, Mounter};
use crate::driver::volume::XetCSIVolume;

mod mount;
pub(crate) mod volume;

#[derive(Debug)]
pub(crate) struct XetHubCSIDriver {
    // map of volume_id to XetCSIVolume
    // TODO: make this map be backed by file system for resilience
    volumes: Mutex<HashMap<String, XetCSIVolume>>,
    mounter: Box<dyn Mounter>,
}

impl XetHubCSIDriver {
    pub fn new() -> Self {
        XetHubCSIDriver {
            volumes: Mutex::new(HashMap::new()),
            mounter: Box::new(GitXetMounter),
        }
    }

    pub(crate) async fn publish(&self, volume_spec: XetCSIVolume) -> Result<(), Status> {
        if !volume_spec.validate() {
            return Err(Status::invalid_argument("invalid volume spec"));
        }
        let mut volumes = self.volumes.lock().await;
        if let Some(volume) = volumes.get(&volume_spec.volume_id) {
            return if &volume_spec == volume {
                // repeat request, already published
                Ok(())
            } else {
                // incompatible
                Err(Status::already_exists(format!("volume {} already exists", volume_spec.volume_id)))
            };
        }
        self.mounter.mount(&volume_spec).await.map_err(|e| {
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
        self.mounter.unmount(volume.path.clone()).await.map_err(|e| {
            warn!("error in umount: {e:?}");
            Status::internal(e)
        })?;
        let _ = volumes.remove(volume_id.as_str());
        Ok(())
    }
}

mod test {
    use std::collections::HashMap;
    use tokio::sync::Mutex;
    use tonic::{Code, Status};
    use crate::driver::mount::{MockMounter, Mounter};
    use crate::driver::volume::{XetCSIVolume};
    use crate::driver::XetHubCSIDriver;
    use crate::error::K8sCSIXetFSError;

    impl XetHubCSIDriver {
        fn new_for_test(mounter: Box<dyn Mounter>) -> Self {
            Self {
                volumes: Mutex::new(HashMap::new()),
                mounter,
            }
        }
    }

    fn spec<T: Into<String>>(volume_id: T, repo: T, commit: T, path: T) -> XetCSIVolume {
        XetCSIVolume {
            volume_id: volume_id.into(),
            path: path.into(),
            repo: repo.into(),
            commit: commit.into(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_simple() {
        let mut mounter = MockMounter::new();
        mounter.expect_mount().times(1).returning(|_| Ok(()));
        let driver = XetHubCSIDriver::new_for_test(Box::new(mounter));
        let volume_spec = spec("123", "123", "123", "123");
        assert!(driver.publish(volume_spec.clone()).await.is_ok());
        assert_eq!(driver.volumes.lock().await.get("123").unwrap(), &volume_spec);
    }

    #[tokio::test]
    async fn test_repeat_publish() {
        let mut mounter = MockMounter::new();
        // mount should only be called once.
        mounter.expect_mount().times(1).returning(|_| Ok(()));
        let driver = XetHubCSIDriver::new_for_test(Box::new(mounter));
        let volume_spec = spec("123", "123", "123", "123");
        driver.publish(volume_spec.clone()).await.unwrap();
        let result = driver.publish(volume_spec).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_repeat_volume_id() {
        let mut mounter = MockMounter::new();
        // mount should only be called once.
        mounter.expect_mount().times(1).returning(|_| Ok(()));
        let driver = XetHubCSIDriver::new_for_test(Box::new(mounter));
        let volume_spec = spec("123", "123", "123", "123");
        driver.publish(volume_spec.clone()).await.unwrap();
        let volume_spec = spec("123", "124", "124", "124");
        let result = driver.publish(volume_spec).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().code(), Code::AlreadyExists));
    }

    #[tokio::test]
    async fn test_mount_fails() {
        let mut mounter = MockMounter::new();
        mounter.expect_mount().times(1).returning(|_| Err(K8sCSIXetFSError::IOError(std::io::Error::new(std::io::ErrorKind::NotFound, ""))));
        let driver = XetHubCSIDriver::new_for_test(Box::new(mounter));
        let volume_spec = spec("123", "123", "123", "123");
        let result = driver.publish(volume_spec).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().code(), Code::Internal));
    }

}
