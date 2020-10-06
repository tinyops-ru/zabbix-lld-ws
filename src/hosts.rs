pub mod hosts {
    use serde::Deserialize;
    use serde::Serialize;

    use crate::errors::errors::OperationError;
    use crate::http::http::send_post_request;
    use crate::types::types::OperationResult;
    use crate::zabbix::zabbix;
    use crate::zabbix::zabbix::ZabbixRequest;

    #[derive(Serialize)]
    struct SearchRequest {
        jsonrpc: String,
        method: String,
        params: SearchRequestParams,
        auth: String,
        id: u8
    }

    #[derive(Serialize)]
    struct SearchRequestParams {
        hostids: Vec<String>
    }

    #[derive(Deserialize)]
    struct SearchResponse {
        result: Vec<ZabbixHost>
    }

    #[derive(Deserialize)]
    pub struct ZabbixHost {
        pub hostid: String,
        pub host: String
    }

    pub fn find_hosts(client: &reqwest::blocking::Client,
                      api_endpoint: &str, api_token: &str, ids: Vec<String>) ->
                                                                OperationResult<Vec<ZabbixHost>> {
        info!("find hosts by ids..");

        let params = SearchRequestParams { hostids: ids };

        let request: ZabbixRequest<SearchRequestParams> = ZabbixRequest::new(
            "host.get", params, api_token
        );

        match send_post_request(client, api_endpoint, request) {
            Ok(response) => {
                let search_response: SearchResponse = serde_json::from_str(&response)
                                                .expect(zabbix::UNSUPPORTED_RESPONSE_MESSAGE);
                Ok(search_response.result)
            }
            Err(_) => {
                error!("unable to find zabbix hosts");
                Err(OperationError::Error)
            }
        }
    }
}
