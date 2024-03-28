use std::fmt::Debug;
use crate::error::K8sCSIXetFSError;
use crate::error::K8sCSIXetFSError::GenericError;
use tokio::process::Command;
use async_trait::async_trait;
use tracing::{error, info};
use crate::driver::volume::XetCSIVolume;

const GIT_XET_BIN: &str = "git-xet";
const GIT_XET_MOUNT_SUBCOMMAND: &str = "mount";
const GIT_XET_MOUNT_REF_FLAG: &str = "-r";
const GIT_XET_ENV_VAR_USER_NAME: &str = "XET_USER_NAME";
const GIT_XET_ENV_VAR_USER_TOKEN: &str = "XET_USER_TOKEN";
const UMOUNT_BIN: &str = "umount";

#[mockall::automock]
#[async_trait]
pub(crate) trait Mounter: Send + Sync + Debug {
    async fn mount(&self, volume_spec: &XetCSIVolume) -> Result<(), K8sCSIXetFSError>;
    async fn unmount(&self, path: String) -> Result<(), K8sCSIXetFSError>;
}

#[derive(Debug)]
pub(crate) struct GitXetMounter;

#[async_trait]
impl Mounter for GitXetMounter {
    async fn mount(&self, volume_spec: &XetCSIVolume) -> Result<(), K8sCSIXetFSError> {
        let mut cmd = Command::new(GIT_XET_BIN);
        let args = [
            GIT_XET_MOUNT_SUBCOMMAND,
            GIT_XET_MOUNT_REF_FLAG,
            volume_spec.commit.as_str(),
            volume_spec.repo.as_str(),
            volume_spec.path.as_str(),
        ];
        info!("mount, running {GIT_XET_BIN} with args: {args:?}");
        cmd.args(args);
        cmd.env("XET_LOG_LEVEL", "info");
        if !volume_spec.user.is_empty() && !volume_spec.pat.is_empty() {
            info!("setting user name and token env vars in mount command");
            cmd.env(GIT_XET_ENV_VAR_USER_NAME, volume_spec.user.as_str());
            cmd.env(GIT_XET_ENV_VAR_USER_TOKEN, volume_spec.pat.as_str());
        }
        if let Some(cas) = &volume_spec.cas {
            cmd.env("XET_CAS_SERVER", cas.as_str());
        }

        cmd.spawn()?.wait().await?;

        Ok(())
    }


    async fn unmount(&self, path: String) -> Result<(), K8sCSIXetFSError> {
        let mut cmd = Command::new(UMOUNT_BIN);
        cmd.arg(path);
        let exit_status = cmd.spawn()?.wait().await?;
        if !exit_status.success() {
            error!("umount failed {exit_status}");
            return Err(GenericError(format!(
                "umount existed with status code: {exit_status}"
            )));
        }
        Ok(())
    }
}

