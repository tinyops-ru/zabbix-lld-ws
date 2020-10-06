#[cfg(test)]
mod config_tests {
    use std::path::Path;

    use crate::config::config::load_config_from_file;

    #[test]
    fn complete_config_should_be_loaded_from_file() {
        let file_path = Path::new("tests/wszl.yml");

        match load_config_from_file(file_path) {
            Ok(config) => {
                assert_eq!(config.zabbix.api_endpoint, "http://zabbix/api_jsonrpc.php");
                assert_eq!(config.zabbix.username, "abcd");
                assert_eq!(config.zabbix.password, "0329jg02934jg34g");
            }
            Err(_) => panic!("config should be loaded")
        }
    }
}
