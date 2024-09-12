use crate::config::AppConfig;
use crate::types::OperationResult;
use anyhow::Context;
use config::Config;
use std::path::Path;

pub fn load_config_from_file(file_path: &Path) -> OperationResult<AppConfig> {
    let file_path_str = format!("{}", file_path.display());
    info!("loading config from file '{file_path_str}'");

    let settings = Config::builder()
        .add_source(config::File::with_name(&file_path_str))
        .build()?;

    let config = settings.try_deserialize::<AppConfig>()
        .context("unable to load config")?;

    info!("config loaded: {}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::config::file::load_config_from_file;
    use crate::config::item::ZabbixItemConfig;
    use crate::config::{AppConfig, WebScenarioConfig, ZabbixApiConfig, ZabbixConfig, ZabbixTriggerConfig};
    use std::path::Path;
    use zabbix_api::host::ZabbixHostTag;

    #[test]
    fn complete_config_should_be_loaded_from_file() {
        let file_path = Path::new("test-data/wszl.yml");

        match load_config_from_file(file_path) {
            Ok(config) => {
                let expected_config = AppConfig {
                    zabbix: ZabbixConfig {
                        target_hostname: "test".to_string(),

                        api: ZabbixApiConfig {
                            endpoint: "http://zabbix/api_jsonrpc.php".to_string(),
                            username: "abcd".to_string(),
                            password: "0329jg02934jg34g".to_string(),
                        },

                        item: ZabbixItemConfig {
                            name_template: "Vhost '{}' item".to_string(),
                            key_template: "vhost.item[{}]".to_string(),
                            interface_id: "0".to_string(),
                            delay: "5m".to_string(),
                            r#type: 7,
                            value_type: 0,
                            tags: vec![
                                ZabbixHostTag {
                                    tag: "abc".to_string(),
                                    value: "something".to_string(),
                                }
                            ],
                        },

                        trigger: ZabbixTriggerConfig {
                            name: "Site '${URL}' is unavailable".to_string(),
                            priority: 4,
                            problem_expression: "avg(/${HOST}/web.test.fail[${URL}],#3)>=1".to_string(),
                            recovery_mode: 0,
                            recovery_expression: "last(/${HOST}/web.test.fail[${URL}])=0".to_string(),
                            event_name: "${URL} is down".to_string(),
                            url: "${URL}".to_string(),
                        },

                        scenario: WebScenarioConfig {
                            key_starts_with: "blablabla".to_string(),
                            name_template: "Check index page '${URL}'".to_string(),
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