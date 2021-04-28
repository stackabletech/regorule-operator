use stackable_config::{ConfigOption, Configurable, Configuration};
use std::collections::{HashMap, HashSet};

pub struct RegoRuleConfig {
    pub port: u16,
}

impl RegoRuleConfig {
    pub const PORT: ConfigOption = ConfigOption {
        name: "port",
        default: Some("3030"),
        required: false,
        takes_argument: true,
        help: "The port on which the webserver will listen that serves the OPA management API",
        documentation: "",
        list: false,
    };
}

impl Configurable for RegoRuleConfig {
    fn get_config_description() -> Configuration {
        let mut options = HashSet::new();
        options.insert(RegoRuleConfig::PORT);

        Configuration {
            name: "Stackable RegoRule Operator",
            version: "0.1",
            about: "Manages Rego rules for OpenPolicyAgents",
            options,
        }
    }

    fn parse_values(
        parsed_values: HashMap<ConfigOption, Option<Vec<String>>>,
    ) -> Result<Self, anyhow::Error> {
        let opt = parsed_values
            .get(&Self::PORT)
            .expect("Has a default, can't be None")
            .as_ref()
            .expect("Has a default, can't be None")
            .first()
            .expect("Has a default, can't be None");

        Ok(RegoRuleConfig { port: opt.parse()? })
    }
}
