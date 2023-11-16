use std::collections::HashMap;
use tokio::sync::{RwLock};
use tonic::{async_trait, Request, Response, Status};
use crate::constants::*;
use crate::proto::csi::v1::{
    node_server::Node,
    NodeExpandVolumeRequest, NodeExpandVolumeResponse,
    NodeGetCapabilitiesRequest, NodeGetCapabilitiesResponse, NodeGetInfoRequest,
    NodeGetInfoResponse, NodeGetVolumeStatsRequest, NodeGetVolumeStatsResponse,
    NodePublishVolumeRequest, NodePublishVolumeResponse,
    NodeStageVolumeRequest, NodeStageVolumeResponse, NodeUnpublishVolumeRequest,
    NodeUnpublishVolumeResponse, NodeUnstageVolumeRequest, NodeUnstageVolumeResponse,
};

#[derive(Debug)]
struct XetCSIVolume {
    repo: String,
    volume_id: String,
    // TODO: fill what we need to set up mounts
}

#[derive(Debug, Default)]
pub struct XetHubCSIService {
    // map of volume_id to XetCSIVolume
    // TODO: make this map be backed by file system for resilience
    volumes: RwLock<HashMap<String, XetCSIVolume>>,
}

impl XetHubCSIService {
   pub fn new() -> Self {
       XetHubCSIService {
           volumes: RwLock::new(HashMap::new()),
       }
   }
}

#[inline]
fn missing_capability<T: std::fmt::Display>(capability: T) -> Status {
    Status::failed_precondition(format!("missing capability: {capability}"))
}

#[async_trait]
impl Node for XetHubCSIService {
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
        todo!()
    }

    async fn node_unpublish_volume(
        &self,
        request: Request<NodeUnpublishVolumeRequest>,
    ) -> Result<Response<NodeUnpublishVolumeResponse>, Status> {
        todo!()
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
        request: Request<NodeGetInfoRequest>,
    ) -> Result<Response<NodeGetInfoResponse>, Status> {
        todo!()
    }
}
