pub mod webscenarios {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde::Serialize;

    use crate::zabbix::zabbix::{CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON, JSONRPC};

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
        id: u8
    }

    #[derive(Serialize)]
    struct GetWebScenariosRequestParams {
        search: HashMap<String, String>
    }

    #[derive(Deserialize)]
    struct WebScenariosResponse {
        result: Vec<ZabbixWebScenario>
    }

    #[derive(Serialize)]
    struct CreateRequest {
        jsonrpc: String,
        method: String,
        params: CreateRequestParams,
        auth: String,
        id: u8
    }

    #[derive(Serialize)]
    struct CreateRequestParams {
        name: String,
        hostid: String,
        steps: Vec<WebScenarioStep>
    }

    #[derive(Serialize)]
    struct WebScenarioStep {
        name: String,
        url: String,
        status_codes: String,
        no: u8
    }

    pub fn find_web_scenarios(client: &reqwest::blocking::Client,
                              api_endpoint: &str, auth_token: &str) ->
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

    pub fn create_web_scenario(client: &reqwest::blocking::Client,
                               api_endpoint: &str, auth_token: &str,
                               item_url: &str, host_id: &str) ->
                                                            Result<(), Box<dyn std::error::Error>> {
        println!("creating web scenario for '{}'", item_url);

        let mut search_params = HashMap::new();
        search_params.insert("key_".to_string(), "Check index page '".to_string());

        let scenario_name = format!("Check index page '{}'", item_url);

        let step = WebScenarioStep {
            name: "Get page".to_string(),
            url: item_url.to_string(),
            status_codes: "200".to_string(),
            no: 1
        };

        let request = CreateRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "httptest.create".to_string(),

            params: CreateRequestParams {
                name: scenario_name,
                hostid: host_id.to_string(),
                steps: vec![step]
            },
            auth: auth_token.to_string(),
            id: 1
        };

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

        Ok(())
    }
}
