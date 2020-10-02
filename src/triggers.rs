pub mod triggers {
    use serde::Serialize;
    use crate::zabbix::zabbix::{CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON, JSONRPC};

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
        description: String,
        expression: String
    }

    pub fn create_trigger(api_endpoint: &str, api_token: &str,
                          host: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("create trigger for '{}', url '{}'", host, url);

        let expression_body = format!("{}:web.test.fail[Check index page '{}'].last()", host, url);

        let expression_with_bracket = "{".to_string() + &expression_body;

        let expression = expression_with_bracket + "}<>0";

        let request = CreateRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "trigger.create".to_string(),
            params: CreateRequestParams {
                description: "Check if url available".to_string(),
                expression
            },
            auth: api_token.to_string(),
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

        Ok(())
    }
}
