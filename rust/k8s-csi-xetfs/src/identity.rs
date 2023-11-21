use crate::proto::csi::v1::identity_server::Identity;
use crate::proto::csi::v1::{
    GetPluginCapabilitiesRequest, GetPluginCapabilitiesResponse,
    GetPluginInfoRequest, GetPluginInfoResponse, ProbeRequest, ProbeResponse,
};
use crate::constants::PLUGIN_NAME;
use tonic::{async_trait, Request, Response, Status};
use tracing::info;

pub struct IdentityService;

#[async_trait]
impl Identity for IdentityService {
    async fn get_plugin_info(
        &self,
        _request: Request<GetPluginInfoRequest>,
    ) -> Result<Response<GetPluginInfoResponse>, Status> {
        info!("GetPluginInfo");
        let get_plugin_info_response = GetPluginInfoResponse {
            name: PLUGIN_NAME.to_owned(),
            vendor_version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        };

        Ok(Response::new(get_plugin_info_response))
    }

    async fn get_plugin_capabilities(
        &self,
        _request: Request<GetPluginCapabilitiesRequest>,
    ) -> Result<Response<GetPluginCapabilitiesResponse>, Status> {
        info!("GetPluginCapabilities");
        let get_plugin_capabilities_response = GetPluginCapabilitiesResponse {
            capabilities: vec![],
        };

        Ok(Response::new(get_plugin_capabilities_response))
    }

    async fn probe(
        &self,
        _request: Request<ProbeRequest>,
    ) -> Result<Response<ProbeResponse>, Status> {
        info!("Probe");
        Ok(Response::new(Default::default()))
    }
}
