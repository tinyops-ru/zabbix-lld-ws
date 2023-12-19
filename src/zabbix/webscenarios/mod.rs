use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct ZabbixWebScenario {
    pub name: String
}

#[derive(Serialize)]
pub struct GetSearchRequestParams {
    pub search: HashMap<String, String>
}

#[derive(Deserialize)]
pub struct WebScenariosResponse {
    pub result: Option<Vec<ZabbixWebScenario>>
}

#[derive(Serialize)]
pub struct CreateRequestParams {
    pub name: String,
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub steps: Vec<WebScenarioStep>,
    pub delay: String,
    pub retries: u8
}

#[derive(Serialize)]
pub struct WebScenarioStep {
    pub name: String,
    pub url: String,
    pub status_codes: String,
    pub no: u8
}

