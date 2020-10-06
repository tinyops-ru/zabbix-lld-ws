pub mod config {
    use std::fs;
    use std::path::Path;

    use yaml_rust::YamlLoader;

    pub struct Config {
        pub zabbix: ZabbixConfig
    }

    pub struct ZabbixConfig {
        pub api_endpoint: String,
        pub username: String,
        pub password: String
    }

    pub fn load_config_from_file(file_path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
        println!("loading config from file '{}'", file_path.display());

        let config_file_content = fs::read_to_string(file_path)?;

        match YamlLoader::load_from_str(&config_file_content) {
            Ok(configs) => {
                let config = &configs[0];

                let zabbix_config = &config["zabbix"];
                let api_endpoint = zabbix_config["api-endpoint"].as_str()
                                               .expect("property 'api-endpoint' wasn't found");
                let username = zabbix_config["username"].as_str()
                                               .expect("property 'username' wasn't found");
                let password = zabbix_config["password"].as_str()
                                               .expect("property 'password' wasn't found");

                println!("config has been loaded");

                Ok(
                    Config {
                        zabbix: ZabbixConfig {
                            api_endpoint: api_endpoint.to_string(),
                            username: username.to_string(),
                            password: password.to_string()
                        }
                    }
                )
            }
            Err(e) =>
                panic!("config file syntax error: {}", e)
        }
    }
}
