use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use semver::{SemVerError, Version};
use serde::{Deserialize, Serialize};
use stackable_operator::Crd;
use std::collections::HashMap;

#[derive(Clone, CustomResource, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[kube(
    group = "opa.stackable.tech",
    version = "v1",
    kind = "OpenPolicyAgent",
    shortname = "opa",
    namespaced
)]
#[kube(status = "OpenPolicyAgentStatus")]
pub struct OpenPolicyAgentSpec {
    pub version: OpaVersion,
    pub servers: Role<OpaConfiguration>,
    pub config: Option<OpaConfiguration>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role<T> {
    pub role_groups: HashMap<String, RoleGroup<T>>,
    pub config: Option<T>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleGroup<T> {
    pub instances: Option<u32>,
    pub instances_per_node: Option<u16>,
    pub config: Option<T>,
    //#[schemars(schema_with = "label_selector_schema")]
    //#pub selector: Option<LabelSelector>,
}

impl Crd for OpenPolicyAgent {
    const RESOURCE_NAME: &'static str = "opa.opa.stackable.tech";
    const CRD_DEFINITION: &'static str = include_str!("../opa.crd.yaml");
}

#[derive(Clone, CustomResource, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
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
    const CRD_DEFINITION: &'static str = include_str!("../opa.crd.yaml");
}

#[allow(non_camel_case_types)]
#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    JsonSchema,
    PartialEq,
    Serialize,
    strum_macros::Display,
    strum_macros::EnumString,
)]
pub enum OpaVersion {
    #[serde(rename = "3.4.14")]
    #[strum(serialize = "3.4.14")]
    v3_4_14,

    #[serde(rename = "3.5.8")]
    #[strum(serialize = "3.5.8")]
    v3_5_8,
}

impl OpaVersion {
    pub fn is_valid_upgrade(&self, to: &Self) -> Result<bool, SemVerError> {
        let from_version = Version::parse(&self.to_string())?;
        let to_version = Version::parse(&to.to_string())?;

        Ok(to_version > from_version)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpaConfiguration {}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPolicyAgentStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<OpaVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<OpaVersion>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(schema_with = "stackable_operator::conditions::schema")]
    pub conditions: Vec<Condition>,
}

impl OpenPolicyAgentStatus {
    pub fn target_image_name(&self) -> Option<String> {
        self.target_version.as_ref().map(|version| {
            format!(
                "stackable/opa:{}",
                serde_json::json!(version).as_str().unwrap()
            )
        })
    }
}
