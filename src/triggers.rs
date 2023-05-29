pub mod triggers {
    use serde::Deserialize;
    use serde::Serialize;

    use crate::errors::errors::OperationError;
    use crate::http::http::send_post_request;
    use crate::types::types::EmptyResult;
    use crate::zabbix::zabbix::{log_zabbix_error, ZabbixError, ZabbixRequest};
    use crate::zabbix::zabbix;

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
                          host: &str, url: &str) -> EmptyResult {
        debug!("create trigger for '{}', url '{}'", host, url);

        let expression = format!("last(/{}/web.test.fail[Check index page '{}'])<>0", host, url);

        let trigger_name = format!("Site '{}' is unavailable", url);

        let params = CreateRequestParams {
            description: trigger_name,
            expression,
            priority: "4".to_string(),
            url: url.to_string()
        };

        let request: ZabbixRequest<CreateRequestParams> = ZabbixRequest::new(
            "trigger.create", params, api_token
        );

        match send_post_request(client, api_endpoint, request) {
            Ok(response) => {
                let create_response: CreateTriggerResponse = serde_json::from_str(&response)
                    .expect(zabbix::UNSUPPORTED_RESPONSE_MESSAGE);

                match create_response.error {
                    Some(_) => {
                        log_zabbix_error(&create_response.error);
                        error!("unable to create trigger for '{}'", url);
                        Err(OperationError::Error)
                    }
                    None => {
                        info!("trigger has been created for url '{}'", url);
                        Ok(())
                    }
                }
            }
            Err(_) => {
                error!("unable to find zabbix items");
                Err(OperationError::Error)
            }
        }
    }
}
