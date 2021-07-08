use stackable_operator::crd::CustomResourceExt;
use stackable_regorule_crd::RegoRule;

fn main() {
    let target_file = "deploy/crd/regorule.crd.yaml";
    RegoRule::write_yaml_schema(target_file).unwrap();
    println!("Wrote CRD to [{}]", target_file);
}
