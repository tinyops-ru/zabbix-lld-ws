use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Context;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::config::{ZabbixConfig, ZabbixTriggerConfig};
use crate::http::send_post_request;
use crate::template::{get_template_vars, process_template_string};
use crate::types::{EmptyResult, OperationResult};
use crate::webscenarios::GetSearchRequestParams;
use crate::zabbix::{log_zabbix_error, UNSUPPORTED_RESPONSE_MESSAGE, ZABBIX_API_COMMUNICATION_ERROR, ZabbixError, ZabbixRequest};

#[derive(Deserialize, Clone, Debug)]
pub struct ZabbixTrigger {
    pub name: String
}

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

pub fn find_zabbix_trigger(client: &Client, zabbix_config: &ZabbixConfig,
                       auth_token: &str, name: &str) ->
                       OperationResult<Option<ZabbixTrigger>> {
    info!("find trigger by name '{name}'..");

    let mut search_params = HashMap::new();
    search_params.insert("description".to_string(), name.to_string());

    let params = GetSearchRequestParams {
        search: search_params
    };

    let request: ZabbixRequest<GetSearchRequestParams> = ZabbixRequest::new(
        "trigger.get", params, auth_token
    );

    let response = send_post_request(client, &zabbix_config.api.endpoint, request)
        .context(ZABBIX_API_COMMUNICATION_ERROR)?;

    let search_response: TriggerSearchResponse = serde_json::from_str(&response)
        .context(UNSUPPORTED_RESPONSE_MESSAGE)?;

    if !search_response.result.is_empty() {
        let trigger = search_response.result.first().unwrap();
        debug!("trigger found: {:?}", &trigger);
        Ok(Some(trigger.clone()))

    } else {
        info!("trigger wasn't found by name '{name}'");
        Ok(None)
    }
}

#[derive(Deserialize)]
struct TriggerSearchResponse {
    result: Vec<ZabbixTrigger>,
    error: Option<ZabbixError>
}

pub fn create_trigger(client: &Client,
                      api_endpoint: &str, api_token: &str,
                      trigger: &ZabbixTriggerConfig,
                      host: &str, url: &str) -> EmptyResult {

    debug!("create trigger for host '{host}', url '{url}'");
    debug!("trigger config '{:?}'", trigger);

    let template_vars = get_template_vars(&host, &url);

    let trigger_name = process_template_string(&trigger.name, &template_vars);
    let expression = process_template_string(&trigger.value, &template_vars);

    debug!("trigger name '{trigger_name}'");
    debug!("trigger expression '{expression}'");

    let params = CreateRequestParams {
        description: trigger_name,
        expression,
        priority: "4".to_string(),
        url: url.to_string()
    };

    let request: ZabbixRequest<CreateRequestParams> = ZabbixRequest::new(
        "trigger.create", params, api_token
    );

    let response = send_post_request(client, api_endpoint, request)
                                    .context(ZABBIX_API_COMMUNICATION_ERROR)?;

    let create_response: CreateTriggerResponse = serde_json::from_str(&response)
                                                .context(UNSUPPORTED_RESPONSE_MESSAGE)?;

    match create_response.error {
        Some(_) => {
            log_zabbix_error(&create_response.error);
            error!("unable to create trigger for '{url}'");
            Err(anyhow!("unable to create trigger for url"))
        }
        None => {
            info!("trigger has been created for url '{url}'");
            Ok(())
        }
    }

}

