use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ZabbixTriggerConfig {
    pub name: String,
    pub value: String,
}

impl Display for ZabbixTriggerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name '{}', value '{}'", self.name, self.value)
    }
}