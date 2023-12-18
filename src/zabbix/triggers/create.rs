use anyhow::{anyhow, Context};
use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

use crate::config::{ZabbixConfig, ZabbixTriggerConfig};
use crate::http::send_post_request;
use crate::template::{get_template_vars, process_template_string};
use crate::types::EmptyResult;
use crate::zabbix::{log_zabbix_error, UNSUPPORTED_RESPONSE_MESSAGE, ZABBIX_API_COMMUNICATION_ERROR, ZabbixError, ZabbixRequest};
use crate::zabbix::hosts::ZabbixHost;
use crate::zabbix::triggers::find::find_zabbix_trigger;

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

pub fn create_trigger_if_does_not_exists(zabbix_config: &ZabbixConfig, auth_token: &str,
                                     url: &str, client: &Client,
                                     zabbix_host: &ZabbixHost) -> EmptyResult {
    let template_vars = get_template_vars(&zabbix_host.host, &url);
    let trigger_name = process_template_string(
        &zabbix_config.trigger.name, &template_vars);

    match find_zabbix_trigger(&client, &zabbix_config, &auth_token, &trigger_name) {
        Ok(trigger) => {
            if trigger.is_none() {
                match create_trigger(&client,
                                     &zabbix_config.api.endpoint, &auth_token,
                                     &zabbix_config.trigger, &zabbix_host.host, &url) {
                    Ok(_) => info!("trigger '{trigger_name}' has been created"),
                    Err(e) =>
                        error!("unable to create trigger '{trigger_name}': {}", e)
                }

            } else {
                info!("trigger '{trigger_name}' already exists, skip")
            }
        }
        Err(e) =>
            error!("unable to find zabbix trigger by name '{trigger_name}': {}", e)
    }

    Ok(())
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