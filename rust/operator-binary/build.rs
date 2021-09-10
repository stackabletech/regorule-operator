use stackable_operator::crd::CustomResourceExt;
use stackable_regorule_crd::RegoRule;

fn main() -> Result<(), stackable_operator::error::Error> {
    built::write_built_file().expect("Failed to acquire build-time information");

    RegoRule::write_yaml_schema("../../deploy/crd/regorule.crd.yaml")?;

    Ok(())
}
