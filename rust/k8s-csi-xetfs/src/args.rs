use clap::Parser;

/// Must pass --endpoint <endpoint> and --node-id <id> args
#[derive(Debug, Clone, Parser)]
pub struct DriverArgs {
    #[clap(long)]
    pub node_id: String,
    #[clap(long)]
    pub endpoint: String,
}