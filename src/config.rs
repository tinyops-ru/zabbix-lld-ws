pub mod config {
    use std::fs;
    use std::path::Path;

    use yaml_rust::YamlLoader;

    use crate::errors::errors::OperationError;
    use crate::types::types::OperationResult;

    pub struct Config {
        pub zabbix: ZabbixConfig
    }

    pub struct ZabbixConfig {
        pub api: ZabbixApiConfig,
        pub scenario: WebScenarioConfig
    }

    pub struct ZabbixApiConfig {
        pub endpoint: String,
        pub username: String,
        pub password: String
    }

    pub struct WebScenarioConfig {
        pub response_timeout: String,
        pub expected_status_code: String,
        pub attempts: u8,
        pub update_interval: String
    }

    pub fn load_config_from_file(file_path: &Path) -> OperationResult<Config> {
        info!("loading config from file '{}'", file_path.display());

        let config_file_content = fs::read_to_string(file_path)?;

        match YamlLoader::load_from_str(&config_file_content) {
            Ok(configs) => {
                let config = &configs[0];

                let zabbix_config = &config["zabbix"];

                let zabbix_api_config = &zabbix_config["api"];

                let api_endpoint = zabbix_api_config["endpoint"].as_str()
                                               .expect("property 'endpoint' wasn't found");
                let username = zabbix_api_config["username"].as_str()
                                               .expect("property 'username' wasn't found");
                let password = zabbix_api_config["password"].as_str()
                                               .expect("property 'password' wasn't found");

                let web_scenario_config = &zabbix_config["scenario"];

                let response_timeout = web_scenario_config["response-timeout"].as_str()
                                        .expect("property 'response-timeout' wasn't found");

                let expected_status_code = web_scenario_config["expected-status-code"].as_str()
                                            .expect("property 'expected-status-code' wasn't found");

                let attempts = web_scenario_config["attempts"].as_i64()
                    .expect("property 'attempts' wasn't found");

                let update_interval = web_scenario_config["update-interval"].as_str()
                    .expect("property 'update-interval' wasn't found");

                info!("config has been loaded");

                Ok(
                    Config {
                        zabbix: ZabbixConfig {
                            api: ZabbixApiConfig {
                                endpoint: api_endpoint.to_string(),
                                username: username.to_string(),
                                password: password.to_string()
                            },
                            scenario: WebScenarioConfig {
                                response_timeout: response_timeout.to_string(),
                                expected_status_code: expected_status_code.to_string(),
                                attempts: attempts as u8,
                                update_interval: update_interval.to_string()
                            }
                        }
                    }
                )
            }
            Err(e) => {
                error!("unable to load config from file: {}", e);
                Err(OperationError::Error)
            }
        }
    }
}
