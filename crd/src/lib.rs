use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The spec for a RegoRule only has a single field: `rego`.
///
/// The string provided should be a complete and valid Rego rule.
/// This means it also needs to specify a package name.
#[derive(
    Clone, CustomResource, Debug, Default, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize,
)]
#[kube(
    group = "opa.stackable.tech",
    version = "v1alpha1",
    kind = "RegoRule",
    shortname = "rego",
    plural = "regorules",
    namespaced
)]
pub struct RegoRuleSpec {
    pub rego: String,
}
