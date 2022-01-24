use flate2::write::GzEncoder;
use flate2::Compression;
use futures::StreamExt;
use snafu::{OptionExt, ResultExt, Snafu};
use stackable_operator::client::Client;
use stackable_operator::error::OperatorResult;
use stackable_operator::kube::core::params::ListParams;
use stackable_operator::kube::runtime::reflector::store::Writer;
use stackable_operator::kube::runtime::reflector::Store;
use stackable_operator::kube::runtime::{reflector, utils};
use stackable_operator::kube::Api;
use stackable_operator::kube::ResourceExt;
use stackable_operator::namespace::WatchNamespace;
use stackable_regorule_crd::RegoRule;
use std::fs::File;
use std::future::Future;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tar::{Builder, Header};
use tracing::{info, trace, warn};
use warp::Filter;

#[derive(Snafu, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[snafu(display("io error occured: {}", source))]
    IoError { source: std::io::Error },
    #[snafu(display("object [{}] needs to be namespaced!", obj))]
    NoNamespace { obj: String },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn rebuild_bundle(reader: &Store<RegoRule>) -> Result<()> {
    // This will be the timestamp for all files in our bundle
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock went backwards")
        .as_secs();

    let path = "/tmp/bundle.tar.gz";
    let tar_gz = File::create(path).context(IoError)?;
    let gz_encoder = GzEncoder::new(tar_gz, Compression::best());
    let mut tar_builder = Builder::new(gz_encoder);

    // Each RegoRule will be its own file in the bundle.
    // The name of the file will be the name of the object with ".rego" appended.
    // TODO: Need to make sure that all Kubernetes names are also valid file names
    for rule in reader.state() {
        let name = format!("{}.rego", rule.name());
        let namespace = rule.namespace().context(NoNamespace { obj: rule.name() })?;

        let path = Path::new(&namespace).join(name);

        let data = rule.spec.rego.as_bytes();

        // This is the tar header that is required for every entry in a tar file
        let mut header = Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o440);
        header.set_mtime(time);

        tar_builder
            .append_data(&mut header, path, data)
            .context(IoError)?;
    }

    tar_builder.finish().context(IoError)?;

    Ok(())
}

/// This creates a Controller which watches `RegoRule`s in Kubernetes.
///
/// This is an async method and the returned future needs to be consumed to make progress.
async fn create_controller(client: Client, namespace: WatchNamespace) -> impl Future<Output = ()> {
    let api: Api<RegoRule> = namespace.get_api(&client);
    let store: Writer<RegoRule> = reflector::store::Writer::<RegoRule>::default();
    let reader: Store<RegoRule> = store.as_reader();
    let rf = reflector(
        store,
        stackable_operator::kube::runtime::watcher(api, ListParams::default()),
    );

    // need to run/drain the reflector - so utilize the for_each to rebuild the bundle files
    utils::try_flatten_touched(rf)
        // Convert from Result<> to Option<>, discarding any Err values
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(move |resource| {
            trace!("Touched {:?}", resource.name());
            // TODO: Later allow to configure what should happen in case of a failure
            rebuild_bundle(&reader).expect("Building the bundle failed, panicing!");
            futures::future::ready(())
        })
}

async fn create_server(port: u16) -> impl Future<Output = ()> {
    // TODO: Support ETag
    // TODO: Support TLS
    // TODO: Support configuring the listening address
    let bundle = warp::path!("opa" / "v1" / "opa" / "bundle.tar.gz")
        .and(warp::fs::file("/tmp/bundle.tar.gz"));
    let bundle = bundle.with(warp::log("bundle"));
    warp::serve(bundle).run(([0, 0, 0, 0], port))
}

pub async fn run_reflector_and_server(
    client: Client,
    namespace: WatchNamespace,
    port: u16,
) -> OperatorResult<()> {
    let reflector = create_controller(client, namespace).await;
    let web_server = create_server(port).await;

    tokio::select! {
        _ = web_server => info!("warp exited"),
        _ = reflector => warn!("reflector drained"),
    }

    Ok(())
}
