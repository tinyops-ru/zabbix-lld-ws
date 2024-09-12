use std::collections::HashMap;

pub const HOST_TEMPLATE_VAR: &str = "HOST";
pub const URL_TEMPLATE_VAR: &str = "URL";
pub const URL_WITHOUT_PROTOCOL_TEMPLATE_VAR: &str = "URL_WITHOUT_PROTOCOL";

pub fn process_template_string(input: &str, template_vars: &HashMap<String, String>) -> String {
    let mut result: String = input.to_string();

    for (key, value) in template_vars {
        let key = format!("${{{}}}", key);
        result = result.replace(&key, &value);
    }

    result.to_string()
}

pub fn get_template_vars(host: &str, url: &str) -> HashMap<String, String> {
    let url_without_protocol = url.replace("https://", "")
                                         .replace("http://", "");

    HashMap::from([
        (HOST_TEMPLATE_VAR.to_string(), host.to_string()),
        (URL_TEMPLATE_VAR.to_string(), url.to_string()),
        (URL_WITHOUT_PROTOCOL_TEMPLATE_VAR.to_string(), url_without_protocol),
    ])
}

#[cfg(test)]
mod template_tests {
    use std::collections::HashMap;

    use crate::template::{get_template_vars, process_template_string, HOST_TEMPLATE_VAR, URL_TEMPLATE_VAR, URL_WITHOUT_PROTOCOL_TEMPLATE_VAR};

    const EXAMPLE_INPUT: &str = "This is a ${HOST}, url check ${URL}. Without url ${URL_WITHOUT_PROTOCOL}.";

    #[test]
    fn template_vars_must_be_present() {
        let vars = get_template_vars("demo", "https://zabbix.com");

        assert!(vars.contains_key(HOST_TEMPLATE_VAR));
        assert!(vars.contains_key(URL_TEMPLATE_VAR));
        assert!(vars.contains_key(URL_WITHOUT_PROTOCOL_TEMPLATE_VAR));

        assert_eq!(vars.get(HOST_TEMPLATE_VAR).unwrap(), "demo");
        assert_eq!(vars.get(URL_TEMPLATE_VAR).unwrap(), "https://zabbix.com");
        assert_eq!(vars.get(URL_WITHOUT_PROTOCOL_TEMPLATE_VAR).unwrap(), "zabbix.com");
    }

    #[test]
    fn template_vars_should_be_resolved() {
        let hostname = "demo";
        let url = "https://zabbix.com";
        let url_without_protocol = "zabbix.com";

        let template_vars: HashMap<String, String> = HashMap::from([
            (HOST_TEMPLATE_VAR.to_string(), hostname.to_string()),
            (URL_TEMPLATE_VAR.to_string(), url.to_string()),
            (URL_WITHOUT_PROTOCOL_TEMPLATE_VAR.to_string(), url_without_protocol.to_string())
        ]);

        let result = process_template_string(&EXAMPLE_INPUT, &template_vars);

        assert_eq!(result, "This is a demo, url check https://zabbix.com. Without url zabbix.com.".to_string())
    }

    #[test]
    fn unknown_vars_should_be_ignored() {
        let hostname = "demo";

        let template_vars: HashMap<String, String> = HashMap::from([
            (HOST_TEMPLATE_VAR.to_string(), hostname.to_string())
        ]);

        let result = process_template_string(&EXAMPLE_INPUT, &template_vars);

        assert_eq!(result, "This is a demo, url check ${URL}. Without url ${URL_WITHOUT_PROTOCOL}.".to_string())
    }
}