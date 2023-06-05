use anyhow::anyhow;
use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::config::ZabbixTriggerConfig;
use crate::http::send_post_request;
use crate::template::{get_template_vars, process_template_string};
use crate::types::EmptyResult;
use crate::zabbix::{log_zabbix_error, UNSUPPORTED_RESPONSE_MESSAGE, ZABBIX_API_COMMUNICATION_ERROR, ZabbixError, ZabbixRequest};

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
                      trigger: &ZabbixTriggerConfig,
                      host: &str, url: &str) -> EmptyResult {

    debug!("create trigger for host '{host}', url '{url}'");
    debug!("trigger config '{:?}'", trigger);

    let template_vars = get_template_vars(&host, &url);

    let trigger_name = process_template_string(&trigger.name, &template_vars);
    let trigger_expression = process_template_string(&trigger.value, &template_vars);

    let expression_with_bracket = "{".to_string() + &trigger_expression;
    let expression = expression_with_bracket + "}<>0";

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

