use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use anyhow::anyhow;
use anyhow::Context;

use crate::config::WebScenarioConfig;
use crate::http::send_post_request;
use crate::types::{EmptyResult, OperationResult};
use crate::zabbix::{log_zabbix_error, UNSUPPORTED_RESPONSE_MESSAGE, ZabbixError, ZabbixRequest};

#[derive(Deserialize, Debug)]
pub struct ZabbixWebScenario {
    pub name: String
}

#[derive(Serialize)]
struct GetWebScenariosRequestParams {
    search: HashMap<String, String>
}

#[derive(Deserialize)]
struct WebScenariosResponse {
    result: Option<Vec<ZabbixWebScenario>>,
    error: Option<ZabbixError>
}

#[derive(Serialize)]
struct CreateRequestParams {
    name: String,
    hostid: String,
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

    let params = GetWebScenariosRequestParams {
        search: search_params
    };

    let request: ZabbixRequest<GetWebScenariosRequestParams> = ZabbixRequest::new(
        "httptest.get", params, auth_token
    );

    let response = send_post_request(client, api_endpoint, request).context("api communication error")?;

    let search_response: WebScenariosResponse = serde_json::from_str(&response)
        .context(UNSUPPORTED_RESPONSE_MESSAGE)?;

    if let Some(web_scenarios) = search_response.result {
        debug!("web scenarios found: {:?}", web_scenarios);
        Ok(web_scenarios)

    } else {
      Err(anyhow!("unable to load web scenarios"))
    }
}

pub fn create_web_scenario(client: &reqwest::blocking::Client,
                           api_endpoint: &str, auth_token: &str,
                           scenario_config: &WebScenarioConfig,
                           item_url: &str, host_id: &str) -> EmptyResult {

    info!("creating web scenario for '{item_url}'");
    debug!("host-id: '{host_id}'");

    let mut search_params = HashMap::new();
    search_params.insert("key_".to_string(), "Check index page '".to_string());

    let scenario_name = format!("Check index page '{item_url}'");

    let step = WebScenarioStep {
        name: "Get page".to_string(),
        url: item_url.to_string(),
        status_codes: scenario_config.expect_status_code.to_string(),
        no: 1
    };

    let params = CreateRequestParams {
        name: scenario_name,
        hostid: host_id.to_string(),
        delay: scenario_config.update_interval.to_string(),
        retries: scenario_config.attempts,
        steps: vec![step],
    };


    let request: ZabbixRequest<CreateRequestParams> = ZabbixRequest::new(
        "httptest.create", params, auth_token
    );

    send_post_request(client, api_endpoint, request).context("unable to create web scenario")?;
    info!("web scenario has been created for '{item_url}'");

    Ok(())
}