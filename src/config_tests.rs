#[cfg(test)]
mod config_tests {
    use std::path::Path;

    use crate::config::config::load_config_from_file;

    #[test]
    fn complete_config_should_be_loaded_from_file() {
        let file_path = Path::new("tests/wszl.yml");

        match load_config_from_file(file_path) {
            Ok(config) => {
                assert_eq!(config.zabbix.api.endpoint, "http://zabbix/api_jsonrpc.php");
                assert_eq!(config.zabbix.api.username, "abcd");
                assert_eq!(config.zabbix.api.password, "0329jg02934jg34g");

                assert_eq!(config.zabbix.scenario.response_timeout, "15s");
                assert_eq!(config.zabbix.scenario.expected_status_code, "200");
                assert_eq!(config.zabbix.scenario.attempts, "3");
                assert_eq!(config.zabbix.scenario.update_interval, "5m");
                assert_eq!(config.zabbix.scenario.application, "SITE");
            }
            Err(_) => panic!("config should be loaded")
        }
    }
}
