pub mod node;
pub mod constants;
pub mod identity;
pub mod error;

pub mod proto {
    pub mod csi {
        pub mod v1 {
            tonic::include_proto!("csi.v1");
        }
    }

    pub mod pluginregistration {
        pub mod v1 {
            tonic::include_proto!("pluginregistration");
        }
    }
}
