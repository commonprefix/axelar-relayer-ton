use dotenv::dotenv;

use relayer_core::config::config_from_yaml;
use relayer_core::heartbeat::heartbeats_loop;
use ton::config::TONConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let network = std::env::var("NETWORK").expect("NETWORK must be set");
    let config: TONConfig = config_from_yaml(&format!("config.{network}.yaml"))?;
    let common_config = config.common_config.clone();

    heartbeats_loop(&common_config).await
}
