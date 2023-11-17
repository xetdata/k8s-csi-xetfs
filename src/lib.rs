use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::constants::TRACING_LEVEL;
use crate::error::K8sCSIXetFSError;
use crate::error::K8sCSIXetFSError::GenericError;

pub mod node;
pub mod constants;
pub mod identity;
pub mod error;
pub mod args;

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

pub fn initialize_tracing_subscriber() -> Result<(), K8sCSIXetFSError> {
    // Logging format
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_line_number(true)
        .with_file(true)
        .with_target(false);

    // Level filter
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(TRACING_LEVEL))
        .map_err(|e| GenericError(e.to_string()))?;

    // Set the global tracing subscriber.
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer.json())
        .try_init()
        .map_err(|e| GenericError(e.to_string()))?;

    Ok(())
}