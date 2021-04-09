use stackable_regorule_crd::RegoRule;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", serde_yaml::to_string(&RegoRule::crd())?);
    Ok(())
}
