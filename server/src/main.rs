mod config;

use crate::config::RegoRuleConfig;
use stackable_config::ConfigBuilder;
use stackable_operator::{client, error};
use std::env;
use std::ffi::OsString;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    stackable_operator::logging::initialize_logging("REGORULE_OPERATOR_LOG");

    let config: RegoRuleConfig =
        ConfigBuilder::build(env::args_os().collect::<Vec<OsString>>(), "CONFIG_FILE")
            .expect("Error initializing Configuration!");

    let client = client::create_client(None).await?;
    let watch_namespace = stackable_operator::namespace::get_watch_namespace()?;
    stackable_regorule_operator::run_reflector_and_server(client, watch_namespace, config.port)
        .await;
    Ok(())
}
