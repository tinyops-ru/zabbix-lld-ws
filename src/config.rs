use std::fmt::{Display, Formatter};
use std::path::Path;

use anyhow::Context;
use config::Config;
use serde::Deserialize;

use crate::types::OperationResult;

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
    pub api: ZabbixApiConfig,

    pub trigger: ZabbixTriggerConfig,

    pub scenario: WebScenarioConfig
}

impl Display for ZabbixConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "api: '{}', scenario: '{}'", self.api, self.scenario)
    }
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ZabbixTriggerConfig {
    pub name: String,
    pub value: String,
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

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WebScenarioConfig {
    pub key_starts_with: String,
    pub name: String,
    pub response_timeout: String,
    pub expect_status_code: String,
    pub attempts: u8,
    pub update_interval: String
}

impl Display for WebScenarioConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "name: '{}', response-timeout: '{}', expect-status-code: '{}, attempts: {}, update-interval: '{}'",
            self.name, self.response_timeout,
            self.expect_status_code, self.attempts, self.update_interval
        )
    }
}

pub fn load_config_from_file(file_path: &Path) -> OperationResult<AppConfig> {
    let file_path_str = format!("{}", file_path.display());
    info!("loading config from file '{file_path_str}'");

    let settings = Config::builder()
        .add_source(config::File::with_name(&file_path_str))
        .build()
        .unwrap();

    let config = settings.try_deserialize::<AppConfig>()
        .context("unable to load config")?;

    info!("config loaded: {}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::config::{AppConfig, load_config_from_file, WebScenarioConfig, ZabbixApiConfig, ZabbixConfig, ZabbixTriggerConfig};

    #[test]
    fn complete_config_should_be_loaded_from_file() {
        let file_path = Path::new("tests/wszl.yml");

        match load_config_from_file(file_path) {
            Ok(config) => {
                let expected_config = AppConfig {
                    zabbix: ZabbixConfig {
                        api: ZabbixApiConfig {
                            endpoint: "http://zabbix/api_jsonrpc.php".to_string(),
                            username: "abcd".to_string(),
                            password: "0329jg02934jg34g".to_string(),
                        },

                        trigger: ZabbixTriggerConfig {
                            name: "Site '${URL}' is unavailable".to_string(),
                            value: "last(/${HOST}/web.test.fail[Check index page '${URL}'])<>0".to_string(),
                        },

                        scenario: WebScenarioConfig {
                            key_starts_with: "blablabla".to_string(),
                            name: "Check index page '${URL}'".to_string(),
                            response_timeout: "15s".to_string(),
                            expect_status_code: "200".to_string(),
                            attempts: 3,
                            update_interval: "5m".to_string(),
                        }
                    },
                };

                assert_eq!(config, expected_config);
            }
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("{}", e.root_cause());
                panic!("config expected");
            }
        }
    }
}
