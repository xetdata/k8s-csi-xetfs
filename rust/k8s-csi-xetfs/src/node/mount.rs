use crate::error::K8sCSIXetFSError;
use crate::error::K8sCSIXetFSError::GenericError;
use crate::node::volume::XetCSIVolume;
use tokio::process::Command;
use tracing::{error, info};

const GIT_XET_BIN: &str = "git-xet";
const GIT_XET_MOUNT_SUBCOMMAND: &str = "mount";
const GIT_XET_MOUNT_REF_FLAG: &str = "-r";
const GIT_XET_ENV_VAR_USER_NAME: &str = "XET_USER_NAME";
const GIT_XET_ENV_VAR_USER_TOKEN: &str = "XET_USER_TOKEN";

pub(crate) async fn mount(volume_spec: &XetCSIVolume) -> Result<(), K8sCSIXetFSError> {
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
    if !volume_spec.user.is_empty() && !volume_spec.pat.is_empty() {
        info!("setting user name and token env vars in mount command");
        cmd.envs([
            ("XET_LOG_LEVEL", "info"),
            (GIT_XET_ENV_VAR_USER_NAME, volume_spec.user.as_str()),
            (GIT_XET_ENV_VAR_USER_TOKEN, volume_spec.pat.as_str()),
        ]);
    }

    cmd.spawn()?.wait().await?;

    Ok(())
}

const UMOUNT_BIN: &str = "umount";

pub(crate) async fn unmount(path: String) -> Result<(), K8sCSIXetFSError> {
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
