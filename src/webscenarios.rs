pub mod webscenarios {
    use std::collections::HashMap;
    use serde::Serialize;
    use serde::Deserialize;
    use crate::zabbix::zabbix::{JSONRPC, CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON};

    #[derive(Deserialize)]
    pub struct ZabbixWebScenario {
        pub name: String
    }

    #[derive(Serialize)]
    struct GetWebScenariosRequest {
        jsonrpc: String,
        method: String,
        params: GetWebScenariosRequestParams,
        auth: String,
        id: i8
    }

    #[derive(Serialize)]
    struct GetWebScenariosRequestParams {
        search: HashMap<String, String>
    }

    #[derive(Deserialize)]
    struct WebScenariosResponse {
        result: Vec<ZabbixWebScenario>
    }

    pub fn find_web_scenarios(api_endpoint: &str, auth_token: &str) ->
                                        Result<Vec<ZabbixWebScenario>, Box<dyn std::error::Error>> {
        println!("searching web scenarios..");

        let mut search_params = HashMap::new();
        search_params.insert("key_".to_string(), "Check index page '".to_string());

        let request = GetWebScenariosRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "httptest.get".to_string(),
            params: GetWebScenariosRequestParams {
                search: search_params
            },
            auth: auth_token.to_string(),
            id: 1
        };

        let client = reqwest::blocking::Client::new();

        let request_body = serde_json::to_string(&request).unwrap();

        let response = client.post(api_endpoint)
            .body(request_body)
            .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON)
            .send()?;

        let response_status = response.status();
        let response_text = response.text().unwrap();

        println!("{}", response_text);

        if response_status != reqwest::StatusCode::OK {
            println!("{}", response_text);
            panic!("unexpected server response code {}", response_status)
        }

        let search_response: WebScenariosResponse = serde_json::from_str(&response_text).unwrap();

        Ok(search_response.result)
    }
}
