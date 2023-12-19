use std::env;

const ENV_WSZL_ZABBIX_API_URL: &str = "WSZL_ZABBIX_API_URL";
const ENV_WSZL_ZABBIX_API_USER: &str = "WSZL_ZABBIX_API_USER";
const ENV_WSZL_ZABBIX_API_PASSWORD: &str = "WSZL_ZABBIX_API_PASSWORD";

const ENV_WSZL_EXAMPLE_HOST_ID: &str = "WSZL_EXAMPLE_HOST_ID";
const ENV_WSZL_EXAMPLE_HOST_NAME: &str = "WSZL_EXAMPLE_HOST_NAME";

pub fn are_integration_tests_enabled() -> bool {
    let result = env::var(ENV_WSZL_ZABBIX_API_URL).is_ok() &&
    env::var(ENV_WSZL_ZABBIX_API_USER).is_ok() &&
    env::var(ENV_WSZL_ZABBIX_API_PASSWORD).is_ok() &&
    env::var(ENV_WSZL_EXAMPLE_HOST_ID).is_ok() &&
    env::var(ENV_WSZL_EXAMPLE_HOST_NAME).is_ok();

    if !result {
        println!("/!\\ integration tests are disabled /!\\")
    }

    result
}

pub struct IntegrationTestsConfig {
    pub zabbix_api_url: String,
    pub zabbix_api_user: String,
    pub zabbix_api_password: String,
    pub example_host_id: String,
    pub example_host_name: String
}

pub fn get_integration_tests_config() -> IntegrationTestsConfig {
    IntegrationTestsConfig {
        zabbix_api_url: env::var(ENV_WSZL_ZABBIX_API_URL).unwrap(),
        zabbix_api_user: env::var(ENV_WSZL_ZABBIX_API_USER).unwrap(),
        zabbix_api_password: env::var(ENV_WSZL_ZABBIX_API_PASSWORD).unwrap(),
        example_host_id: env::var(ENV_WSZL_EXAMPLE_HOST_ID).unwrap(),
        example_host_name: env::var(ENV_WSZL_EXAMPLE_HOST_NAME).unwrap(),
    }
}