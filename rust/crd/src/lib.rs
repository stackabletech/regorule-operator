use serde::{Deserialize, Serialize};
use stackable_operator::kube::CustomResource;
use stackable_operator::schemars::{self, JsonSchema};

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
    namespaced,
    crates(
        kube_core = "stackable_operator::kube::core",
        k8s_openapi = "stackable_operator::k8s_openapi",
        schemars = "stackable_operator::schemars"
    )
)]
pub struct RegoRuleSpec {
    pub rego: String,
}
