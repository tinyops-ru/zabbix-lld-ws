use serde::Deserialize;
use serde::Serialize;

use crate::errors::OperationError;
use crate::http::send_post_request;
use crate::types::EmptyResult;
use crate::zabbix::{log_zabbix_error, UNSUPPORTED_RESPONSE_MESSAGE, ZabbixError, ZabbixRequest};

#[derive(Serialize)]
struct CreateRequestParams {
    description: String,
    expression: String,
    priority: String,
    url: String
}

#[derive(Deserialize)]
struct CreateTriggerResponse {
    error: Option<ZabbixError>
}

pub fn create_trigger(client: &reqwest::blocking::Client,
                      api_endpoint: &str, api_token: &str,
                      host: &str, url: &str) -> EmptyResult {
    debug!("create trigger for '{host}', url '{url}'");

    let expression_body = format!("{host}:web.test.fail[Check index page '{url}'].last()");

    let expression_with_bracket = "{".to_string() + &expression_body;

    let expression = expression_with_bracket + "}<>0";

    let trigger_name = format!("Site '{url}' is unavailable");

    let params = CreateRequestParams {
        description: trigger_name,
        expression,
        priority: "4".to_string(),
        url: url.to_string()
    };

    let request: ZabbixRequest<CreateRequestParams> = ZabbixRequest::new(
        "trigger.create", params, api_token
    );

    match send_post_request(client, api_endpoint, request) {
        Ok(response) => {
            let create_response: CreateTriggerResponse = serde_json::from_str(&response)
                .expect(UNSUPPORTED_RESPONSE_MESSAGE);

            match create_response.error {
                Some(_) => {
                    log_zabbix_error(&create_response.error);
                    error!("unable to create trigger for '{url}'");
                    Err(OperationError::Error)
                }
                None => {
                    info!("trigger has been created for url '{url}'");
                    Ok(())
                }
            }
        }
        Err(_) => {
            error!("unable to find zabbix items");
            Err(OperationError::Error)
        }
    }
}