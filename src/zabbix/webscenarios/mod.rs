use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::http::send_post_request;
use crate::types::OperationResult;
use crate::zabbix::{UNSUPPORTED_RESPONSE_MESSAGE, ZABBIX_API_COMMUNICATION_ERROR, ZabbixRequest};

pub mod create;

#[derive(Deserialize, Debug)]
pub struct ZabbixWebScenario {
    pub name: String
}

#[derive(Serialize)]
pub struct GetSearchRequestParams {
    pub search: HashMap<String, String>
}

#[derive(Deserialize)]
struct WebScenariosResponse {
    result: Option<Vec<ZabbixWebScenario>>
}

#[derive(Serialize)]
struct CreateRequestParams {
    name: String,
    #[serde(rename = "hostid")]
    host_id: String,
    steps: Vec<WebScenarioStep>,
    delay: String,
    retries: u8
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
                          OperationResult<Vec<ZabbixWebScenario>> {
    info!("searching web scenarios..");

    let mut search_params = HashMap::new();
    search_params.insert("key_".to_string(), "Check index page '".to_string());

    let params = GetSearchRequestParams {
        search: search_params
    };

    let request: ZabbixRequest<GetSearchRequestParams> = ZabbixRequest::new(
        "httptest.get", params, auth_token
    );

    let response = send_post_request(client, api_endpoint, request)
        .context(ZABBIX_API_COMMUNICATION_ERROR)?;

    let search_response: WebScenariosResponse = serde_json::from_str(&response)
        .context(UNSUPPORTED_RESPONSE_MESSAGE)?;

    if let Some(web_scenarios) = search_response.result {
        debug!("web scenarios found: {:?}", web_scenarios);
        Ok(web_scenarios)

    } else {
        Err(anyhow!("unable to load web scenarios"))
    }
}

