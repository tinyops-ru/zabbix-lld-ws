use std::collections::HashMap;

use anyhow::Context;
use reqwest::blocking::Client;
use serde_derive::Deserialize;

use crate::config::ZabbixConfig;
use crate::http::send_post_request;
use crate::types::OperationResult;
use crate::zabbix::{UNSUPPORTED_RESPONSE_MESSAGE, ZABBIX_API_COMMUNICATION_ERROR, ZabbixRequest};
use crate::zabbix::triggers::ZabbixTrigger;
use crate::zabbix::webscenarios::GetSearchRequestParams;

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
    result: Vec<ZabbixTrigger>
}