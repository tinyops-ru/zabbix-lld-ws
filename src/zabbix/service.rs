use std::collections::HashMap;

use anyhow::Context;
use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

use crate::config::{WebScenarioConfig, ZabbixApiVersion};
use crate::http::send_post_request;
use crate::template::{get_template_vars, process_template_string};
use crate::types::{EmptyResult, OperationResult};
use crate::zabbix::{JSONRPC, ZABBIX_API_COMMUNICATION_ERROR, ZabbixRequest};
use crate::zabbix::webscenarios::{CreateRequestParams, WebScenarioStep};

pub trait ZabbixService {
    fn get_session(&self, zabbix_api_version: ZabbixApiVersion, username: &str, token: &str) -> OperationResult<String>;
    fn create_web_scenario(&self, auth_token: &str, url: &str, host_id: &str,
                           scenario_config: &WebScenarioConfig) -> EmptyResult;
}

struct DefaultZabbixService {
    zabbix_api_url: String,
    client: Client,
}

impl DefaultZabbixService {
    pub fn new(zabbix_api_url: &str, client: &Client) -> DefaultZabbixService {
        DefaultZabbixService {
            zabbix_api_url: zabbix_api_url.to_string(),
            client: client.clone()
        }
    }
}

impl ZabbixService for DefaultZabbixService {
    fn get_session(&self, zabbix_api_version: ZabbixApiVersion, username: &str, token: &str) -> OperationResult<String> {
        let auth_request = AuthRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "user.login".to_string(),
            params: get_params(&zabbix_api_version, username, &token),
            id: 1,
            auth: None
        };

        let response = send_post_request(&self.client, &self.zabbix_api_url, auth_request)
            .context(ZABBIX_API_COMMUNICATION_ERROR)?;

        let auth_response = serde_json::from_str::<AuthResponse>(&response)
            .context("authentication error")?;

        Ok(String::from(auth_response.result))
    }

    fn create_web_scenario(&self, auth_token: &str, url: &str, host_id: &str,
                           scenario_config: &WebScenarioConfig) -> EmptyResult {
        info!("creating web scenario for url '{url}'");
        debug!("host-id: '{host_id}'");
        debug!("scenario config: '{:?}'", scenario_config);

        let template_vars = get_template_vars(&host_id, &url);
        let scenario_name = process_template_string(&scenario_config.name, &template_vars);

        let step = WebScenarioStep {
            name: "Get page".to_string(),
            url: url.to_string(),
            status_codes: scenario_config.expect_status_code.to_string(),
            no: 1
        };

        let params = CreateRequestParams {
            name: scenario_name,
            host_id: host_id.to_string(),
            delay: scenario_config.update_interval.to_string(),
            retries: scenario_config.attempts,
            steps: vec![step],
        };

        let request: ZabbixRequest<CreateRequestParams> = ZabbixRequest::new(
            "httptest.create", params, auth_token
        );

        send_post_request(&self.client, &self.zabbix_api_url, request).context("unable to create web scenario")?;
        info!("web scenario has been created for '{url}'");

        Ok(())
    }
}

#[derive(Serialize)]
struct AuthRequest {
    jsonrpc: String,
    method: String,
    params: HashMap<String, String>,
    id: i8,
    auth: Option<String>
}

#[derive(Deserialize)]
struct AuthResponse {
    result: String
}

fn get_params(api_version: &ZabbixApiVersion,
              username: &str, password: &str) -> HashMap<String, String> {

    let username_field = match api_version {
        ZabbixApiVersion::V6 => "username",
        ZabbixApiVersion::V5 => "user"
    };

    HashMap::from([
        (username_field.to_string(), username.to_string()),
        ("password".to_string(), password.to_string()),
    ])
}