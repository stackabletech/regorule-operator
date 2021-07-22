mod config;

use crate::config::RegoRuleConfig;
use stackable_config::ConfigBuilder;
use stackable_operator::crd::CustomResourceExt;
use stackable_operator::{client, error};
use stackable_regorule_crd::RegoRule;
use std::env;
use std::ffi::OsString;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    stackable_operator::logging::initialize_logging("REGORULE_OPERATOR_LOG");

    let config: RegoRuleConfig =
        ConfigBuilder::build(env::args_os().collect::<Vec<OsString>>(), "CONFIG_FILE")
            .expect("Error initializing Configuration!");

    info!("Starting Stackable Operator for OpenPolicyAgent Rego rules");

    let client = client::create_client(None).await?;

    if let Err(error) =
        stackable_operator::crd::wait_until_crds_present(&client, vec![&RegoRule::crd_name()], None)
            .await
    {
        error!("Required CRDs missing, aborting: {:?}", error);
        return Err(error);
    };

    let watch_namespace = stackable_operator::namespace::get_watch_namespace()?;
    stackable_regorule_operator::run_reflector_and_server(client, watch_namespace, config.port)
        .await;
    Ok(())
}
