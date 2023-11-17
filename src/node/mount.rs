use std::path::PathBuf;
use gitxetcore::command::mount::{mount_command, MountArgs};
use gitxetcore::config::{ConfigGitPathOption, XetConfig};
use tokio::process::Command;
use crate::error::K8sCSIXetFSError;
use crate::error::K8sCSIXetFSError::GenericError;
use crate::node::volume::XetCSIVolume;

const LOCAL_IP: &str = "127.0.0.1";

/// returns a MountArgs with reasonable default values and values populated from the volume spec.
fn mount_args(volume_spec: &XetCSIVolume) -> MountArgs {
    MountArgs {
        remote: volume_spec.repo.clone(),
        path: Some(PathBuf::from(&volume_spec.path)),
        reference: volume_spec.commit.clone(),
        foreground: false,
        prefetch: 16,
        ip: LOCAL_IP.to_owned(),
        clonepath: None,
        writable: false,
        watch: None,
        invoked_from_python: None,
    }
}

pub(crate) async fn mount(volume_spec: &XetCSIVolume) -> Result<(), K8sCSIXetFSError> {
    // TODO: update xet-core with better ways of initializing/configuring these values.
    let xet_config = XetConfig::new(None, None, ConfigGitPathOption::NoPath)
        .map_err(|e| GenericError(e.to_string()))?;
    let args = mount_args(volume_spec);
    if let Err(e) = mount_command(&xet_config, &args).await {
        eprintln!("mount command failed: {e}");
        return Err(GenericError(e.to_string()));
    }
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
