use std::collections::HashMap;

use anyhow::{anyhow, Context};
use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

use crate::config::{WebScenarioConfig, ZabbixApiVersion};
use crate::http::send_post_request;
use crate::template::{get_template_vars, process_template_string};
use crate::types::{EmptyResult, OperationResult};
use crate::zabbix::{JSONRPC, log_zabbix_error, UNSUPPORTED_RESPONSE_MESSAGE, ZABBIX_API_COMMUNICATION_ERROR, ZabbixRequest};
use crate::zabbix::hosts::{SearchRequestParams, ZabbixHost};
use crate::zabbix::items::ZabbixItem;
use crate::zabbix::search::{ItemSearchParams, ItemSearchResponse};
use crate::zabbix::webscenarios::{CreateRequestParams, GetSearchRequestParams, WebScenariosResponse, WebScenarioStep, ZabbixWebScenario};

const UNABLE_TO_FIND_ZABBIX_HOSTS_ERROR: &str = "unable to find zabbix hosts";

pub trait ZabbixService {
    fn get_session(&self, username: &str, token: &str) -> OperationResult<String>;

    fn find_items(&self, auth_token: &str, key_search_mask: &str) -> OperationResult<Vec<ZabbixItem>>;

    fn find_hosts(&self, auth_token: &str, ids: Vec<String>) -> OperationResult<Vec<ZabbixHost>>;

    fn find_web_scenarios(&self, auth_token: &str) -> OperationResult<Vec<ZabbixWebScenario>>;

    fn create_web_scenario(&self, auth_token: &str, url: &str, host_id: &str,
                           scenario_config: &WebScenarioConfig) -> EmptyResult;
}

pub struct DefaultZabbixService {
    zabbix_api_url: String,
    zabbix_api_version: ZabbixApiVersion,
    client: Client,
}

impl DefaultZabbixService {
    pub fn new(zabbix_api_url: &str, zabbix_api_version: &ZabbixApiVersion,
               client: &Client) -> DefaultZabbixService {
        DefaultZabbixService {
            zabbix_api_url: zabbix_api_url.to_string(),
            zabbix_api_version: zabbix_api_version.clone(),
            client: client.clone()
        }
    }
}

impl ZabbixService for DefaultZabbixService {
    fn get_session(&self, username: &str, token: &str) -> OperationResult<String> {
        let auth_request = AuthRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "user.login".to_string(),
            params: get_params(&self.zabbix_api_version, username, &token),
            id: 1,
            auth: None
        };

        let response = send_post_request(&self.client, &self.zabbix_api_url, auth_request)
            .context(ZABBIX_API_COMMUNICATION_ERROR)?;

        let auth_response = serde_json::from_str::<AuthResponse>(&response)
            .context("authentication error")?;

        Ok(String::from(auth_response.result))
    }

    fn find_items(&self, auth_token: &str, key_search_mask: &str) -> OperationResult<Vec<ZabbixItem>> {
        info!("searching zabbix items..");

        let mut search_params = HashMap::new();
        search_params.insert("key_".to_string(), key_search_mask.to_string());

        let params = ItemSearchParams {
            sortfield: "name".to_string(),
            search: search_params
        };

        let request: ZabbixRequest<ItemSearchParams> = ZabbixRequest::new(
            "item.get", params, auth_token
        );

        let response = send_post_request(&self.client, &self.zabbix_api_url, request)
            .context(ZABBIX_API_COMMUNICATION_ERROR)?;

        let search_response: ItemSearchResponse = serde_json::from_str(&response)
            .context(UNSUPPORTED_RESPONSE_MESSAGE)?;

        if let Some(items) = search_response.result {
            debug!("zabbix items: {:?}", items);
            Ok(items)

        } else {
            Err(anyhow!("unable to get zabbix items"))
        }
    }

    fn find_hosts(&self, auth_token: &str, ids: Vec<String>) -> OperationResult<Vec<ZabbixHost>> {
        info!("find hosts by ids..");

        let params = SearchRequestParams { host_ids: ids };

        let request: ZabbixRequest<SearchRequestParams> = ZabbixRequest::new(
            "host.get", params, auth_token
        );

        match send_post_request(&self.client, &self.zabbix_api_url, request) {
            Ok(response) => {
                let search_response: crate::zabbix::hosts::SearchResponse = serde_json::from_str(&response)
                    .expect(UNSUPPORTED_RESPONSE_MESSAGE);

                match search_response.result {
                    Some(hosts) => Ok(hosts),
                    None => {
                        log_zabbix_error(&search_response.error);
                        error!("{}: empty response", UNABLE_TO_FIND_ZABBIX_HOSTS_ERROR);
                        Err(anyhow!(UNABLE_TO_FIND_ZABBIX_HOSTS_ERROR))
                    }
                }
            }
            Err(e) => {
                error!("{}: {}", UNABLE_TO_FIND_ZABBIX_HOSTS_ERROR, e);
                Err(anyhow!(UNABLE_TO_FIND_ZABBIX_HOSTS_ERROR))
            }
        }
    }


    fn find_web_scenarios(&self, auth_token: &str) -> OperationResult<Vec<ZabbixWebScenario>> {
        info!("searching web scenarios..");

        let mut search_params = HashMap::new();
        search_params.insert("key_".to_string(), "Check index page '".to_string());

        let params = GetSearchRequestParams {
            search: search_params
        };

        let request: ZabbixRequest<GetSearchRequestParams> = ZabbixRequest::new(
            "httptest.get", params, auth_token
        );

        let response = send_post_request(&self.client, &self.zabbix_api_url, request)
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

        send_post_request(&self.client, &self.zabbix_api_url, request)
                            .context("unable to create web scenario")?;
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