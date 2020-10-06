pub mod triggers {
    use serde::Serialize;

    use crate::errors::errors::OperationError;
    use crate::types::types::EmptyResult;
    use crate::zabbix::zabbix::{CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON, ZabbixRequest};

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
        expression: String,
        priority: String,
        url: String
    }

    pub fn create_trigger(client: &reqwest::blocking::Client,
                          api_endpoint: &str, api_token: &str,
                          host: &str, url: &str) -> EmptyResult {
        debug!("create trigger for '{}', url '{}'", host, url);

        let expression_body = format!("{}:web.test.fail[Check index page '{}'].last()", host, url);

        let expression_with_bracket = "{".to_string() + &expression_body;

        let expression = expression_with_bracket + "}<>0";

        let trigger_name = format!("Site '{}' is unavailable", url);

        let params = CreateRequestParams {
            description: trigger_name,
            expression,
            priority: "4".to_string(),
            url: url.to_string()
        };

        let request: ZabbixRequest<CreateRequestParams> = ZabbixRequest::new(
            "trigger.create", params, api_token
        );

        let request_body = serde_json::to_string(&request).unwrap();

        match client.post(api_endpoint)
            .body(request_body)
            .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON)
            .send() {
            Ok(response) => {
                let response_status = response.status();
                let response_text = response.text().unwrap();

                debug!("{}", response_text);

                if response_status == reqwest::StatusCode::OK {
                    Ok(())

                } else {
                    error!("unexpected server response code {}", response_status);
                    Err(OperationError::Error)
                }
            }
            Err(e) => {
                error!("unable to create trigger: '{}'", e);
                Err(OperationError::Error)
            }
        }
    }
}
