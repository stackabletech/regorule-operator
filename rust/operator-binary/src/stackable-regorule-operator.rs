use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use stackable_operator::crd::CustomResourceExt;
use stackable_operator::{cli, client};
use stackable_regorule_crd::RegoRule;
use std::env;
use tracing::error;

mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    stackable_operator::logging::initialize_logging("REGORULE_OPERATOR_LOG");

    // Handle CLI arguments
    let matches = App::new(built_info::PKG_DESCRIPTION)
        .author("Stackable GmbH - info@stackable.de")
        .about(built_info::PKG_DESCRIPTION)
        .version(crate_version!())
        .arg(cli::generate_productconfig_arg())
        .subcommand(
            SubCommand::with_name("crd")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(cli::generate_crd_subcommand::<RegoRule>()),
        )
        .arg(Arg::with_name("port").default_value("3030").required(false).takes_value(true).help("The port on which the webserver will listen that serves the OPA management API"))
        .get_matches();

    if let ("crd", Some(subcommand)) = matches.subcommand() {
        if cli::handle_crd_subcommand::<RegoRule>(subcommand)? {
            return Ok(());
        }
    }

    // Safe unwrap because we define a default
    let port = matches.value_of("port").unwrap().parse()?;

    stackable_operator::utils::print_startup_string(
        built_info::PKG_DESCRIPTION,
        built_info::PKG_VERSION,
        built_info::GIT_VERSION,
        built_info::TARGET,
        built_info::BUILT_TIME_UTC,
        built_info::RUSTC_VERSION,
    );

    let client = client::create_client(None).await?;

    if let Err(error) =
        stackable_operator::crd::wait_until_crds_present(&client, vec![&RegoRule::crd_name()], None)
            .await
    {
        error!("Required CRDs missing, aborting: {:?}", error);
        return Err(error.into());
    };

    let watch_namespace = stackable_operator::namespace::get_watch_namespace()?;
    stackable_regorule_operator::run_reflector_and_server(client, watch_namespace, port).await;
    Ok(())
}
