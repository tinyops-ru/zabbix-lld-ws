use std::collections::HashMap;

pub const HOST_TEMPLATE_VAR: &str = "HOST";
pub const URL_TEMPLATE_VAR: &str = "URL";

pub fn process_template_string(input: &str, template_vars: &HashMap<String, String>) -> String {
    let mut result: String = input.to_string();

    for (key, value) in template_vars {
        let key = format!("${{{}}}", key);
        result = result.replace(&key, &value);
    }

    result.to_string()
}

pub fn get_template_vars(host: &str, url: &str) -> HashMap<String, String> {
    HashMap::from([
        (HOST_TEMPLATE_VAR.to_string(), host.to_string()),
        (URL_TEMPLATE_VAR.to_string(), url.to_string()),
    ])
}

#[cfg(test)]
mod template_tests {
    use std::collections::HashMap;

    use crate::template::{HOST_TEMPLATE_VAR, process_template_string, URL_TEMPLATE_VAR};

    const EXAMPLE_INPUT: &str = "this is a ${HOST}, url check ${URL}.";

    #[test]
    fn template_vars_should_be_resolved() {
        let hostname = "demo";
        let url = "https://zabbix.com";

        let template_vars: HashMap<String, String> = HashMap::from([
            (HOST_TEMPLATE_VAR.to_string(), hostname.to_string()),
            (URL_TEMPLATE_VAR.to_string(), url.to_string())
        ]);

        let result = process_template_string(&EXAMPLE_INPUT, &template_vars);

        assert_eq!(result, "this is a demo, url check https://zabbix.com.".to_string())
    }

    #[test]
    fn unknown_vars_should_be_ignored() {
        let hostname = "demo";

        let template_vars: HashMap<String, String> = HashMap::from([
            (HOST_TEMPLATE_VAR.to_string(), hostname.to_string())
        ]);

        let result = process_template_string(&EXAMPLE_INPUT, &template_vars);

        assert_eq!(result, "this is a demo, url check ${URL}.".to_string())
    }
}