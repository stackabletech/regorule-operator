use stackable_operator::crd::CustomResourceExt;
use stackable_regorule_crd::RegoRule;

fn main() {
    let target_file = "deploy/crd/regorule.crd.yaml";
    match RegoRule::write_yaml_schema(target_file) {
        Ok(_) => println!("Wrote CRD to [{}]", target_file),
        Err(err) => println!("Could not write CRD to [{}]: {:?}", target_file, err),
    }
}
