use std::path::PathBuf;
use gitxetcore::command::mount::{mount_command, MountArgs};
use gitxetcore::config::{ConfigGitPathOption, XetConfig};
use tokio::process::Command;
use crate::error::K8sCSIXetFSError;
use crate::error::K8sCSIXetFSError::GenericError;
use crate::node::volume::XetCSIVolume;

pub(crate) async fn mount(volume_spec: &XetCSIVolume) -> Result<(), K8sCSIXetFSError> {
    // TODO: update xet-core with better ways of initializing/configuring these values.
    let xet_config = XetConfig::new(None, None, ConfigGitPathOption::NoPath)
        .map_err(|e| GenericError(e.to_string()))?;
    let args = MountArgs {
        remote: volume_spec.repo.clone(),
        path: Some(PathBuf::from(&volume_spec.path)),
        reference: volume_spec.commit.clone(),
        foreground: false,
        prefetch: 16,
        ip: "127.0.0.1".to_string(),
        clonepath: None,
        writable: false,
        watch: None,
        invoked_from_python: None,
    };
    mount_command(&xet_config, &args).await.map_err(|_| GenericError("".to_string()))?;
    Ok(())
}

const UMOUNT_BIN: &str = "umount";

pub(crate) async fn unmount(path: String) -> Result<(), K8sCSIXetFSError> {
    let mut cmd = Command::new(UMOUNT_BIN);
    cmd.arg(path);
    let exit_status = cmd.spawn()?.wait().await?;
    if !exit_status.success() {
        return Err(GenericError(format!("umount existed with status code: {exit_status}")))
    }
    Ok(())
}
