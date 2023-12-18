use anyhow::anyhow;
use serde::Deserialize;
use serde::Serialize;

use crate::http::send_post_request;
use crate::types::OperationResult;
use crate::zabbix::{log_zabbix_error, UNSUPPORTED_RESPONSE_MESSAGE, ZabbixError, ZabbixRequest};

const UNABLE_TO_FIND_ZABBIX_HOSTS_ERROR: &str = "unable to find zabbix hosts";

#[derive(Serialize)]
struct SearchRequestParams {
    #[serde(rename = "hostids")]
    host_ids: Vec<String>
}

#[derive(Deserialize)]
struct SearchResponse {
    result: Option<Vec<ZabbixHost>>,
    error: Option<ZabbixError>
}

#[derive(Deserialize)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String
}

pub fn find_hosts(client: &reqwest::blocking::Client,
                  api_endpoint: &str, api_token: &str,
                  ids: Vec<String>) -> OperationResult<Vec<ZabbixHost>> {
    info!("find hosts by ids..");

    let params = SearchRequestParams { host_ids: ids };

    let request: ZabbixRequest<SearchRequestParams> = ZabbixRequest::new(
        "host.get", params, api_token
    );

    match send_post_request(client, api_endpoint, request) {
        Ok(response) => {
            let search_response: SearchResponse = serde_json::from_str(&response)
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