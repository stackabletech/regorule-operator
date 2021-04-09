use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stackable_operator::Crd;

/// The spec for a RegoRule only has a single field: `rego`.
///
/// The string provided should be a complete and valid Rego rule.
/// This means it also needs to specify a package name.
#[derive(
    Clone, CustomResource, Debug, Default, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize,
)]
#[kube(
    group = "opa.stackable.tech",
    version = "v1",
    kind = "RegoRule",
    shortname = "rego",
    namespaced
)]
pub struct RegoRuleSpec {
    pub rego: String,
}

impl Crd for RegoRule {
    const RESOURCE_NAME: &'static str = "regorule.opa.stackable.tech";
    const CRD_DEFINITION: &'static str = include_str!("../../deploy/crd/regorule.crd.yaml");
}
