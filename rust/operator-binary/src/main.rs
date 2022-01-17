mod controller;

use anyhow::anyhow;
use clap::Parser;
use stackable_operator::builder::ObjectMetaBuilder;
use stackable_operator::cli::Command;
use stackable_operator::k8s_openapi::api::core::v1::{Service, ServicePort, ServiceSpec};
use stackable_operator::kube::CustomResourceExt;
use stackable_operator::labels::{APP_INSTANCE_LABEL, APP_NAME_LABEL};
use stackable_regorule_crd::{RegoRule, APP_NAME, APP_PORT};
use std::collections::BTreeMap;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser)]
#[clap(about = built_info::PKG_DESCRIPTION, author = "Stackable GmbH - info@stackable.de")]
struct Opts {
    #[clap(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    stackable_operator::logging::initialize_logging("REGORULE_OPERATOR_LOG");

    let opts = Opts::parse();
    match opts.cmd {
        Command::Crd => println!("{}", serde_yaml::to_string(&RegoRule::crd())?,),
        Command::Run { .. } => {
            stackable_operator::utils::print_startup_string(
                built_info::PKG_DESCRIPTION,
                built_info::PKG_VERSION,
                built_info::GIT_VERSION,
                built_info::TARGET,
                built_info::BUILT_TIME_UTC,
                built_info::RUSTC_VERSION,
            );

            let client = stackable_operator::client::create_client(Some(
                "regorule.stackable.tech".to_string(),
            ))
            .await?;

            let cluster_ip = create_cluster_ip_service();
            if let Err(err) = client.create(&cluster_ip).await {
                return Err(anyhow!(
                    "Could not create ClusterIP service: {}",
                    err.to_string()
                ));
            }

            let watch_namespace = stackable_operator::namespace::get_watch_namespace()?;
            controller::run_reflector_and_server(client, watch_namespace, APP_PORT).await?;
        }
    };

    Ok(())
}

pub fn create_cluster_ip_service() -> Service {
    let mut selector = BTreeMap::new();
    selector.insert(APP_INSTANCE_LABEL.to_string(), APP_NAME.to_string());
    selector.insert(APP_NAME_LABEL.to_string(), APP_NAME.to_string());

    Service {
        metadata: ObjectMetaBuilder::new()
            .name(APP_NAME)
            // TODO: namespace?
            .namespace("default")
            // TODO: What would be the ownerreference here?
            //.ownerreference_from_resource(regorule, None, Some(true))
            //.context(ObjectMissingMetadataForOwnerRef)?
            .build(),
        spec: Some(ServiceSpec {
            ports: Some(vec![ServicePort {
                name: Some("http".to_string()),
                port: APP_PORT.into(),
                protocol: Some("TCP".to_string()),
                ..ServicePort::default()
            }]),
            selector: Some(selector),
            type_: Some("ClusterIP".to_string()),
            ..ServiceSpec::default()
        }),
        ..Service::default()
    }
}
