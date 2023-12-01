use crate::proto::csi::v1::identity_server::Identity;
use crate::proto::csi::v1::{
    GetPluginCapabilitiesRequest, GetPluginCapabilitiesResponse,
    GetPluginInfoRequest, GetPluginInfoResponse, ProbeRequest, ProbeResponse,
};
use crate::constants::PLUGIN_NAME;
use tonic::{async_trait, Request, Response, Status};
use tracing::{debug, info};

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
        debug!("Probe");
        Ok(Response::new(Default::default()))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use tonic::Request;
    use crate::constants::PLUGIN_NAME;
    use crate::identity::IdentityService;
    use crate::proto::csi::v1::{GetPluginCapabilitiesRequest, GetPluginInfoRequest, PluginCapability, ProbeRequest};
    use crate::proto::csi::v1::identity_server::Identity;
    use crate::proto::csi::v1::plugin_capability::Type;

    fn get() -> IdentityService {
        IdentityService
    }

    #[tokio::test]
    async fn test_probe() {
        let i = get();
        assert!(i.probe(Request::new(ProbeRequest {})).await.is_ok());
    }

    impl From<PluginCapability> for String {
        fn from(value: PluginCapability) -> Self {
            match value.r#type.unwrap() {
                Type::Service(service) =>     service.r#type.to_string(),
                Type::VolumeExpansion(volume_expansion) => volume_expansion.r#type.to_string(),
            }
        }
    }

    #[tokio::test]
    async fn test_get_plugin_capabilities() {
        let supported_capabilities: HashSet<String> = HashSet::new(); // empty

        let i = get();
        let response = i
            .get_plugin_capabilities(Request::new(GetPluginCapabilitiesRequest{}))
            .await
            .unwrap()
            .into_inner();
        let mut response_capabilities : HashSet<String>= HashSet::new();
        let original_len = response.capabilities.len();
        for capability in response.capabilities {
            response_capabilities.insert(capability.into());
        }
        // assert unique elements
        assert_eq!(original_len, response_capabilities.len());
        // assert expected elements
        assert_eq!(response_capabilities, supported_capabilities);
    }

    #[tokio::test]
    async fn test_get_plugin_info() {
        let i = get();
        let response = i
            .get_plugin_info(Request::new(GetPluginInfoRequest {}))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.name.as_str(), PLUGIN_NAME);
        assert_eq!(response.vendor_version, env!("CARGO_PKG_VERSION"));
    }

}
