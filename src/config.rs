use std::fmt::{Display, Formatter};
use std::path::Path;

use serde::Deserialize;

use config::Config;

use crate::types::OperationResult;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub zabbix: ZabbixConfig
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ZabbixConfig {
    pub api: ZabbixApiConfig,
    pub scenario: WebScenarioConfig
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
        write!(f, "endpoint: '{}', username: '{}', password: '*********'", self.endpoint, self.username)
    }
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WebScenarioConfig {
    pub response_timeout: String,
    pub expect_status_code: String,
    pub attempts: u8,
    pub update_interval: String
}

pub fn load_config_from_file(file_path: &Path) -> OperationResult<AppConfig> {
    info!("loading config from file '{}'", file_path.display());

    let file_path_str = format!("{}", file_path.display());

    let settings = Config::builder()
        .add_source(config::File::with_name(&file_path_str))
        .build()
        .unwrap();

    let config = settings.try_deserialize::<AppConfig>().unwrap();

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::config::{AppConfig, load_config_from_file, WebScenarioConfig, ZabbixApiConfig, ZabbixConfig};

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
                        }, scenario: WebScenarioConfig {
                            response_timeout: "15s".to_string(),
                            expect_status_code: "200".to_string(),
                            attempts: 3,
                            update_interval: "5m".to_string(),
                        }
                    },
                };

                assert_eq!(config, expected_config);
            }
            Err(_) => panic!("config should be loaded")
        }
    }
}
