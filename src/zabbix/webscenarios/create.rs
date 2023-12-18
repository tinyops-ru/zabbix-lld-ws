use std::collections::HashMap;

use anyhow::Context;
use reqwest::blocking::Client;

use crate::config::{WebScenarioConfig, ZabbixConfig};
use crate::http::send_post_request;
use crate::template::{get_template_vars, process_template_string};
use crate::types::EmptyResult;
use crate::zabbix::hosts::ZabbixHost;
use crate::zabbix::webscenarios::{CreateRequestParams, WebScenarioStep};
use crate::zabbix::ZabbixRequest;
use crate::ZabbixEntities;

pub fn create_web_scenario_if_does_not_exists(zabbix_config: &ZabbixConfig, auth_token: &str,
                                          url: &str, client: &Client,
                                          zabbix_host: &ZabbixHost, zabbix_objects: &ZabbixEntities) -> EmptyResult {

    let scenario_name = format!("Check index page '{url}'");

    match zabbix_objects.web_scenarios.iter()
        .find(|entity| entity.name == scenario_name) {
        None => {
            match create_web_scenario(
                &client, &zabbix_config.api.endpoint, &auth_token,
                &zabbix_config.scenario, &url, &zabbix_host.host_id) {
                Ok(_) => info!("web scenario has been created for '{url}'"),
                Err(e) => {
                    error!("unable to create web scenario: {}", e);
                    return Err(e)
                }
            }
        }
        Some(_) => info!("web scenario '{scenario_name}' already found, skip.")
    }

    Ok(())
}

pub fn create_web_scenario(client: &Client,
                           api_endpoint: &str, auth_token: &str,
                           scenario_config: &WebScenarioConfig,
                           item_url: &str, host_id: &str) -> EmptyResult {

    info!("creating web scenario for '{item_url}'");
    debug!("host-id: '{host_id}'");

    let mut search_params = HashMap::new();
    search_params.insert("key_".to_string(), "Check index page '".to_string());

    let template_vars = get_template_vars(&host_id, &item_url);
    let scenario_name = process_template_string(&scenario_config.name, &template_vars);

    let step = WebScenarioStep {
        name: "Get page".to_string(),
        url: item_url.to_string(),
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

    send_post_request(client, api_endpoint, request).context("unable to create web scenario")?;
    info!("web scenario has been created for '{item_url}'");

    Ok(())
}