use crate::proto::csi::v1::{
    node_server::Node, NodeExpandVolumeRequest, NodeExpandVolumeResponse,
    NodeGetCapabilitiesRequest, NodeGetCapabilitiesResponse, NodeGetInfoRequest,
    NodeGetInfoResponse, NodeGetVolumeStatsRequest, NodeGetVolumeStatsResponse,
    NodePublishVolumeRequest, NodePublishVolumeResponse, NodeStageVolumeRequest,
    NodeStageVolumeResponse, NodeUnpublishVolumeRequest, NodeUnpublishVolumeResponse,
    NodeUnstageVolumeRequest, NodeUnstageVolumeResponse,
};
use tonic::{async_trait, Request, Response, Status};
use tracing::{info};
use crate::driver::volume::XetCSIVolume;
use crate::driver::XetHubCSIDriver;
use crate::proto::csi::v1::node_service_capability::rpc::Type;

#[derive(Debug)]
pub struct NodeService {
    driver: XetHubCSIDriver,
    node_id: String,
}

impl NodeService {
    pub fn new(node_id: String) -> Self {
        NodeService {
            node_id,
            driver: XetHubCSIDriver::new(),
        }
    }
}

/// implementation of the gRPC Node service.
/// errors returned from unimplemented stubs are as specified in the CSI Spec.
/// https://github.com/container-storage-interface/spec/blob/master/spec.md#node-service-rpc
#[async_trait]
impl Node for NodeService {
    async fn node_stage_volume(
        &self,
        _request: Request<NodeStageVolumeRequest>,
    ) -> Result<Response<NodeStageVolumeResponse>, Status> {
        missing_capability(Type::StageUnstageVolume, Status::failed_precondition)
    }

    async fn node_unstage_volume(
        &self,
        _request: Request<NodeUnstageVolumeRequest>,
    ) -> Result<Response<NodeUnstageVolumeResponse>, Status> {
        missing_capability(Type::StageUnstageVolume, Status::failed_precondition)
    }

    async fn node_publish_volume(
        &self,
        request: Request<NodePublishVolumeRequest>,
    ) -> Result<Response<NodePublishVolumeResponse>, Status> {
        info!("got publish request: {request:?}");
        let volume_spec: XetCSIVolume = request.into_inner().try_into()?;
        self.driver.publish(volume_spec).await?;
        Ok(Response::new(NodePublishVolumeResponse {}))
    }

    async fn node_unpublish_volume(
        &self,
        request: Request<NodeUnpublishVolumeRequest>,
    ) -> Result<Response<NodeUnpublishVolumeResponse>, Status> {
        info!("got unpublish request: {request:?}");
        let inner = request.into_inner();
        let volume_id = inner.volume_id;
        self.driver.unpublish(volume_id).await?;
        Ok(Response::new(NodeUnpublishVolumeResponse {}))
    }

    async fn node_get_volume_stats(
        &self,
        _request: Request<NodeGetVolumeStatsRequest>,
    ) -> Result<Response<NodeGetVolumeStatsResponse>, Status> {
        missing_capability(Type::GetVolumeStats, Status::failed_precondition)
    }

    async fn node_expand_volume(
        &self,
        _request: Request<NodeExpandVolumeRequest>,
    ) -> Result<Response<NodeExpandVolumeResponse>, Status> {
        missing_capability(Type::ExpandVolume, Status::invalid_argument)
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

fn missing_capability<T>(capability: Type, status: fn(String) -> Status) -> Result<T, Status> {
    Err(status(format!("missing capability {}", capability.as_str_name())))
}
