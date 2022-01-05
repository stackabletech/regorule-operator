mod controller;

use stackable_operator::cli::Command;
use stackable_operator::kube::CustomResourceExt;
use stackable_regorule_crd::RegoRule;
use structopt::StructOpt;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(StructOpt)]
#[structopt(about = built_info::PKG_DESCRIPTION, author = "Stackable GmbH - info@stackable.de")]
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    stackable_operator::logging::initialize_logging("REGORULE_OPERATOR_LOG");

    let opts = Opts::from_args();
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

            let watch_namespace = stackable_operator::namespace::get_watch_namespace()?;
            controller::run_reflector_and_server(client, watch_namespace, 3030).await?;
        }
    };

    Ok(())
}
