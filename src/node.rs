use crate::constants::*;
use crate::proto::csi::v1::{
    node_server::Node, NodeExpandVolumeRequest, NodeExpandVolumeResponse,
    NodeGetCapabilitiesRequest, NodeGetCapabilitiesResponse, NodeGetInfoRequest,
    NodeGetInfoResponse, NodeGetVolumeStatsRequest, NodeGetVolumeStatsResponse,
    NodePublishVolumeRequest, NodePublishVolumeResponse, NodeStageVolumeRequest,
    NodeStageVolumeResponse, NodeUnpublishVolumeRequest, NodeUnpublishVolumeResponse,
    NodeUnstageVolumeRequest, NodeUnstageVolumeResponse,
};
use std::collections::HashMap;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};
use volume::XetCSIVolume;
use crate::node::mount::{mount, unmount};

mod mount;
mod volume;

#[derive(Debug, Default)]
pub struct XetHubCSIDriver {
    node_id: String,
    // map of volume_id to XetCSIVolume
    // TODO: make this map be backed by file system for resilience
    volumes: Mutex<HashMap<String, XetCSIVolume>>,
}

impl XetHubCSIDriver {
    pub fn new(node_id: String) -> Self {
        XetHubCSIDriver {
            node_id,
            volumes: Mutex::new(HashMap::new()),
        }
    }
}

#[inline]
fn missing_capability<T: std::fmt::Display>(capability: T) -> Status {
    Status::failed_precondition(format!("missing capability: {capability}"))
}

#[async_trait]
impl Node for XetHubCSIDriver {
    async fn node_stage_volume(
        &self,
        _request: Request<NodeStageVolumeRequest>,
    ) -> Result<Response<NodeStageVolumeResponse>, Status> {
        Err(missing_capability(CAPABILITY_STAGE_UNSTAGE_VOLUME))
    }

    async fn node_unstage_volume(
        &self,
        _request: Request<NodeUnstageVolumeRequest>,
    ) -> Result<Response<NodeUnstageVolumeResponse>, Status> {
        Err(missing_capability(CAPABILITY_STAGE_UNSTAGE_VOLUME))
    }

    async fn node_publish_volume(
        &self,
        request: Request<NodePublishVolumeRequest>,
    ) -> Result<Response<NodePublishVolumeResponse>, Status> {
        let volume_spec: XetCSIVolume = request.into_inner().try_into()?;

        let mut volumes = self.volumes.lock().await;
        if volumes.contains_key(&volume_spec.volume_id) {
            return Err(Status::already_exists("volume already exists"));
        }
        if let Err(e) = mount(&volume_spec).await {
            return Err(Status::internal(e));
        }
        volumes.insert(volume_spec.volume_id.clone(), volume_spec);

        Ok(Response::new(NodePublishVolumeResponse {}))
    }

    async fn node_unpublish_volume(
        &self,
        request: Request<NodeUnpublishVolumeRequest>,
    ) -> Result<Response<NodeUnpublishVolumeResponse>, Status> {
        let inner = request.into_inner();
        let path = inner.target_path;
        let volume_id = inner.volume_id;

        let mut volumes = self.volumes.lock().await;
        let volume = if let Some(volume) = volumes.get(volume_id.as_str()) {
            volume
        } else {
            return Err(Status::not_found(format!("volume with volume id {volume_id} not found")));
        };
        if volume.path != path {
            // TODO: use tracing::warn
            eprintln!("WARN: paths don't match request {path} got {}", volume.path);
        }
        if let Err(e) = unmount(volume.path.clone()).await {
            return Err(Status::internal(e));
        }
        let _ = volumes.remove(volume_id.as_str());

        Ok(Response::new(NodeUnpublishVolumeResponse {}))
    }

    async fn node_get_volume_stats(
        &self,
        _request: Request<NodeGetVolumeStatsRequest>,
    ) -> Result<Response<NodeGetVolumeStatsResponse>, Status> {
        Err(missing_capability(CAPABILITY_GET_VOLUME_STATS))
    }

    async fn node_expand_volume(
        &self,
        _request: Request<NodeExpandVolumeRequest>,
    ) -> Result<Response<NodeExpandVolumeResponse>, Status> {
        Err(Status::invalid_argument("not supported"))
    }

    async fn node_get_capabilities(
        &self,
        _request: Request<NodeGetCapabilitiesRequest>,
    ) -> Result<Response<NodeGetCapabilitiesResponse>, Status> {
        let node_get_capabilities_response = NodeGetCapabilitiesResponse {
            capabilities: vec![],
        };
        Ok(Response::new(node_get_capabilities_response))
    }

    async fn node_get_info(
        &self,
        _request: Request<NodeGetInfoRequest>,
    ) -> Result<Response<NodeGetInfoResponse>, Status> {
        let node_get_info_response = NodeGetInfoResponse {
            node_id: self.node_id.clone(),
            ..Default::default()
        };
        Ok(Response::new(node_get_info_response))
    }
}
