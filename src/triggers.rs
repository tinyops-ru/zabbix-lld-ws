pub mod triggers {
    use serde::Serialize;

    use crate::errors::errors::OperationError;
    use crate::http::http::send_post_request;
    use crate::types::types::EmptyResult;
    use crate::zabbix::zabbix::ZabbixRequest;

    #[derive(Serialize)]
    struct CreateRequestParams {
        description: String,
        expression: String,
        priority: String,
        url: String
    }

    pub fn create_trigger(client: &reqwest::blocking::Client,
                          api_endpoint: &str, api_token: &str,
                          host: &str, url: &str) -> EmptyResult {
        debug!("create trigger for '{}', url '{}'", host, url);

        let expression_body = format!("{}:web.test.fail[Check index page '{}'].last()", host, url);

        let expression_with_bracket = "{".to_string() + &expression_body;

        let expression = expression_with_bracket + "}<>0";

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
            Ok(_) => {
                info!("trigger has been created for url '{}'", url);
                Ok(())
            }
            Err(_) => {
                error!("unable to find zabbix items");
                Err(OperationError::Error)
            }
        }
    }
}
