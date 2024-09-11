pub mod item;
pub mod file;
pub mod trigger;
pub mod ws;

use std::fmt::{Display, Formatter};

use crate::config::item::ZabbixItemConfig;
use crate::config::trigger::ZabbixTriggerConfig;
use crate::config::ws::WebScenarioConfig;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub zabbix: ZabbixConfig
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.zabbix)
    }
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ZabbixConfig {
    /// Override where web-scenarios and triggers will be created.
    /// Default: the same host where vhosts were located
    #[serde(default = "get_empty_string_value")]
    pub target_hostname: String,

    pub api: ZabbixApiConfig,

    pub item: ZabbixItemConfig,

    pub trigger: ZabbixTriggerConfig,

    pub scenario: WebScenarioConfig
}

impl Display for ZabbixConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "api: '{}', item: '{}', trigger: '{}', scenario: '{}'",
           self.api, self.item, self.trigger, self.scenario
        )
    }
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ZabbixApiConfig {
    pub endpoint: String,
    pub username: String,
    pub password: String
}

impl Display for ZabbixApiConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "endpoint '{}', username '{}', password '***********'", self.endpoint, self.username)
    }
}

fn get_empty_string_value() -> String {
    String::new()
}